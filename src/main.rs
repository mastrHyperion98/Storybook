use iced::{Application, Command, Settings};

mod project;
use project::{Project, ProjectList};
use std::path::PathBuf;

struct Storybook {
    view: AppView,
    project_list: ProjectList,
    current_project: Option<Project>,
}

enum AppView {
    ProjectManagement,
    MainWorkspace,
}

#[derive(Debug, Clone)]
enum Message {
    ProjectsLoaded(Result<ProjectList, String>),
    CreateProject(String, PathBuf),
    LoadProject(PathBuf),
    DeleteProject(PathBuf),
    ProjectCreated(Result<Project, String>),
    ProjectLoaded(Result<Project, String>),
}

impl Application for Storybook {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let cmd = Command::perform(
            async { ProjectList::load() },
            |result| Message::ProjectsLoaded(result.map_err(|e| e.to_string())),
        );

        (
            Storybook {
                view: AppView::ProjectManagement,
                project_list: ProjectList::default(),
                current_project: None,
            },
            cmd,
        )
    }

    fn title(&self) -> String {
        match &self.current_project {
            Some(project) => format!("Storybook - {}", project.name),
            None => String::from("Storybook - Project Management"),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ProjectsLoaded(Ok(list)) => {
                self.project_list = list;
                if let Some(recent) = self.project_list.most_recent() {
                    if recent.is_available() {
                        let project = recent.clone();
                        let path = project.path.clone();
                        self.project_list.update_last_opened(&path);
                        let _ = self.project_list.save();
                        return Command::perform(
                            async move { Ok(project) },
                            Message::ProjectLoaded,
                        );
                    }
                }
                Command::none()
            }
            Message::ProjectsLoaded(Err(_)) => Command::none(),
            Message::CreateProject(name, base_path) => {
                let path = base_path.join(&name);
                let project = Project::new(name, path);
                Command::perform(
                    async move {
                        project.initialize()?;
                        Ok(project)
                    },
                    |result: Result<Project, Box<dyn std::error::Error>>| {
                        Message::ProjectCreated(result.map_err(|e| e.to_string()))
                    },
                )
            }
            Message::ProjectCreated(Ok(project)) => {
                self.project_list.add_project(project.clone());
                let _ = self.project_list.save();
                self.current_project = Some(project);
                self.view = AppView::MainWorkspace;
                Command::none()
            }
            Message::ProjectCreated(Err(_)) => Command::none(),
            Message::LoadProject(path) => {
                if let Some(project) =
                    self.project_list.projects.iter().find(|p| p.path == path)
                {
                    if project.is_available() {
                        let project = project.clone();
                        self.project_list.update_last_opened(&path);
                        let _ = self.project_list.save();
                        self.current_project = Some(project);
                        self.view = AppView::MainWorkspace;
                    }
                }
                Command::none()
            }
            Message::DeleteProject(path) => {
                self.project_list.remove_project(&path);
                let _ = self.project_list.save();
                Command::none()
            }
            Message::ProjectLoaded(Ok(project)) => {
                self.current_project = Some(project);
                self.view = AppView::MainWorkspace;
                Command::none()
            }
            Message::ProjectLoaded(Err(_)) => Command::none(),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        match self.view {
            AppView::ProjectManagement => self.view_project_management(),
            AppView::MainWorkspace => self.view_main_workspace(),
        }
    }
}

impl Storybook {
    fn view_project_management(&self) -> iced::Element<Message> {
        use iced::widget::{button, column, container, scrollable, text, Space};
        use iced::{Alignment, Length};

        let title = text("Storybook - Project Management").size(32);

        let create_button = button(text("+ Create New Project")).padding(10);

        let projects_grid = self.view_projects_grid();

        let content = column![
            title,
            Space::with_height(20),
            create_button,
            Space::with_height(30),
            projects_grid,
        ]
        .padding(40)
        .align_items(Alignment::Start);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_projects_grid(&self) -> iced::Element<Message> {
        use iced::widget::{column, row};

        let sorted_projects = self.project_list.sorted_by_recent();

        let mut rows_vec = Vec::new();
        let mut current_row = Vec::new();

        for (i, project) in sorted_projects.iter().enumerate() {
            current_row.push(self.view_project_card(project));

            if (i + 1) % 2 == 0 || i == sorted_projects.len() - 1 {
                let row_elements = current_row.drain(..).collect::<Vec<_>>();
                rows_vec.push(row(row_elements).spacing(16).into());
            }
        }

        column(rows_vec).spacing(16).into()
    }

    fn view_project_card(&self, project: &Project) -> iced::Element<Message> {
        use iced::widget::{button, column, container, row, text, Space};
        use iced::{Alignment, Length};

        let is_available = project.is_available();

        let name_text = text(&project.name).size(18);

        let time_ago = format_time_ago(&project.last_opened);
        let timestamp_text = text(format!("Last opened: {}", time_ago)).size(14);

        let path_text = text(project.path.display().to_string()).size(12);

        let delete_button = button(text("×").size(20))
            .on_press(Message::DeleteProject(project.path.clone()));

        let mut header_items = vec![
            name_text.into(),
            Space::with_width(Length::Fill).into(),
        ];
        
        if !is_available {
            header_items.push(text("⚠️").size(16).into());
        }
        
        header_items.push(delete_button.into());
        
        let header = row(header_items).align_items(Alignment::Center);

        let card_content =
            column![header, timestamp_text, path_text,].spacing(8).padding(16);

        let card = container(card_content)
            .width(Length::Fixed(350.0))
            .height(Length::Fixed(120.0));

        if is_available {
            button(card)
                .on_press(Message::LoadProject(project.path.clone()))
                .into()
        } else {
            card.into()
        }
    }

    fn view_main_workspace(&self) -> iced::Element<Message> {
        use iced::widget::{container, text};
        use iced::Length;

        if let Some(project) = &self.current_project {
            container(text(format!("Workspace: {}", project.name)).size(24))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        } else {
            container(text("No project loaded").size(24))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        }
    }
}

fn format_time_ago(datetime: &chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(*datetime);

    if duration.num_days() > 0 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{} minutes ago", duration.num_minutes())
    } else {
        "Just now".to_string()
    }
}

fn main() -> iced::Result {
    Storybook::run(Settings::default())
}
