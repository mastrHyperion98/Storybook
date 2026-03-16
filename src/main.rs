use iced::Task;
use iced_aw::{menu_bar, menu_items};
use iced_aw::menu::Menu;

mod project;
use project::{Project, ProjectList};
use std::path::PathBuf;

struct Storybook {
    view: AppView,
    project_list: ProjectList,
    current_project: Option<Project>,
    show_create_dialog: bool,
    new_project_name: String,
    new_project_path: String,
    create_error: Option<String>,
}

enum AppView {
    ProjectManagement,
    MainWorkspace,
}

#[derive(Debug, Clone)]
enum Message {
    ProjectsLoaded(Result<ProjectList, String>),
    ShowCreateDialog,
    HideCreateDialog,
    ProjectNameChanged(String),
    ProjectPathChanged(String),
    CreateProject,
    LoadProject(PathBuf),
    DeleteProject(PathBuf),
    ProjectCreated(Result<Project, String>),
    ProjectLoaded(Result<Project, String>),
    CloseProject,
}

impl Storybook {
    fn new() -> (Self, Task<Message>) {
        let cmd = Task::future(async { ProjectList::load() })
            .map(|result| Message::ProjectsLoaded(result.map_err(|e| e.to_string())));

        (
            Storybook {
                view: AppView::ProjectManagement,
                project_list: ProjectList::default(),
                current_project: None,
                show_create_dialog: false,
                new_project_name: String::new(),
                new_project_path: Project::default_base_path()
                    .to_string_lossy()
                    .to_string(),
                create_error: None,
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

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ProjectsLoaded(Ok(list)) => {
                self.project_list = list;
                if let Some(recent) = self.project_list.most_recent() {
                    if recent.is_available() {
                        let project = recent.clone();
                        let path = project.path.clone();
                        self.project_list.update_last_opened(&path);
                        let _ = self.project_list.save();
                        return Task::done(Message::ProjectLoaded(Ok(project)));
                    }
                }
                Task::none()
            }
            Message::ProjectsLoaded(Err(_)) => Task::none(),
            Message::ShowCreateDialog => {
                self.show_create_dialog = true;
                self.new_project_name = String::new();
                self.new_project_path = Project::default_base_path()
                    .to_string_lossy()
                    .to_string();
                self.create_error = None;
                Task::none()
            }
            Message::HideCreateDialog => {
                self.show_create_dialog = false;
                self.create_error = None;
                Task::none()
            }
            Message::ProjectNameChanged(name) => {
                self.new_project_name = name;
                self.create_error = None;
                Task::none()
            }
            Message::ProjectPathChanged(path) => {
                self.new_project_path = path;
                self.create_error = None;
                Task::none()
            }
            Message::CreateProject => {
                if !Project::validate_name(&self.new_project_name) {
                    self.create_error = Some(
                        "Invalid project name. Use only letters, numbers, hyphens, and underscores."
                            .to_string(),
                    );
                    return Task::none();
                }

                let name = self.new_project_name.clone();
                let base_path = PathBuf::from(&self.new_project_path);
                let path = base_path.join(&name);
                let project = Project::new(name, path);
                Task::future(async move {
                    project.initialize()?;
                    Ok(project)
                }).map(|result: Result<Project, Box<dyn std::error::Error + Send>>| {
                    Message::ProjectCreated(result.map_err(|e| e.to_string()))
                })
            }
            Message::ProjectCreated(Ok(project)) => {
                self.project_list.add_project(project.clone());
                let _ = self.project_list.save();
                self.current_project = Some(project);
                self.view = AppView::MainWorkspace;
                self.show_create_dialog = false;
                self.new_project_name = String::new();
                self.create_error = None;
                Task::none()
            }
            Message::ProjectCreated(Err(e)) => {
                self.create_error = Some(e);
                Task::none()
            }
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
                Task::none()
            }
            Message::DeleteProject(path) => {
                self.project_list.remove_project(&path);
                let _ = self.project_list.save();
                Task::none()
            }
            Message::ProjectLoaded(Ok(project)) => {
                self.current_project = Some(project);
                self.view = AppView::MainWorkspace;
                Task::none()
            }
            Message::ProjectLoaded(Err(_)) => Task::none(),
            Message::CloseProject => {
                self.current_project = None;
                self.view = AppView::ProjectManagement;
                Task::none()
            }
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

        let create_button = button(text("+ Create New Project"))
            .padding(10)
            .on_press(Message::ShowCreateDialog);

        let projects_grid = self.view_projects_grid();

        let content = column![
            title,
            Space::new().height(20),
            create_button,
            Space::new().height(30),
            projects_grid,
        ]
        .padding(40)
        .align_x(Alignment::Start);

        let base_view = container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill);

        if self.show_create_dialog {
            self.view_create_dialog_overlay(base_view.into())
        } else {
            base_view.into()
        }
    }

    fn view_create_dialog_overlay(
        &self,
        _base: iced::Element<Message>,
    ) -> iced::Element<Message> {
        use iced::widget::{button, column, container, row, text, text_input, Space};
        use iced::{Alignment, Length};

        let dialog_title = text("Create New Project").size(24);

        let name_label = text("Project Name:").size(14);
        let name_input = text_input(
            "my-project",
            &self.new_project_name,
        )
        .on_input(Message::ProjectNameChanged)
        .padding(10);

        let path_label = text("Base Path:").size(14);
        let path_input = text_input(
            "/home/user/storybook",
            &self.new_project_path,
        )
        .on_input(Message::ProjectPathChanged)
        .padding(10);

        let preview_path = if !self.new_project_name.is_empty() {
            format!(
                "Full path: {}/{}",
                self.new_project_path, self.new_project_name
            )
        } else {
            "Full path: (enter project name)".to_string()
        };
        let preview_text = text(preview_path).size(12);

        let error_message = if let Some(err) = &self.create_error {
            column![
                text(err).size(14),
                Space::new().height(10),
            ]
        } else {
            column![]
        };

        let create_btn = button(text("Create"))
            .padding(10)
            .on_press(Message::CreateProject);

        let cancel_btn = button(text("Cancel"))
            .padding(10)
            .on_press(Message::HideCreateDialog);

        let buttons = row![create_btn, Space::new().width(10), cancel_btn]
            .spacing(10);

        let dialog_content = column![
            dialog_title,
            Space::new().height(20),
            name_label,
            name_input,
            Space::new().height(15),
            path_label,
            path_input,
            Space::new().height(10),
            preview_text,
            Space::new().height(20),
            error_message,
            buttons,
        ]
        .padding(30)
        .align_x(Alignment::Start)
        .width(Length::Fixed(500.0));

        let dialog = container(dialog_content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill);

        dialog.into()
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

    fn view_project_card<'a>(&'a self, project: &'a Project) -> iced::Element<'a, Message> {
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
            Space::new().width(Length::Fill).into(),
        ];
        
        if !is_available {
            header_items.push(text("⚠️").size(16).into());
        }
        
        header_items.push(delete_button.into());
        
        let header = row(header_items).align_y(Alignment::Center);

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
        use iced::widget::{column, container, text, Space};
        use iced::{Border, Length};

        let menubar = self.view_menubar();

        let content = if let Some(project) = &self.current_project {
            column![
                text(format!("Project: {}", project.name)).size(24),
                text("Main workspace content area").size(16),
            ]
            .padding(20)
            .spacing(10)
        } else {
            column![text("No project loaded").size(24)].padding(20)
        };

        let main_content = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);
        
        // Create a thin separator line
        let separator = container(Space::new().height(0))
            .width(Length::Fill)
            .style(|_theme| container::Style {
                border: Border {
                    color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            });

        let layout = column![menubar, separator, main_content]
            .width(Length::Fill)
            .height(Length::Fill);

        layout.into()
    }

    fn view_menubar(&self) -> iced::Element<Message> {
        use iced::widget::{button, container, row, text};
        use iced::{Alignment, Length};

        let close_project_btn = button(
            text("Close Project")
                .width(Length::Fill)
                .align_y(Alignment::Center)
        )
        .width(Length::Fill)
        .on_press(Message::CloseProject);

        let file_menu = Menu::new(menu_items!(
            (close_project_btn)
        ))
        .width(180.0)
        .offset(0.0)
        .spacing(0.0);

        let mb = menu_bar!(
            (text("File"), file_menu)
        );

        let menubar_row = row![
            mb,
            text("Edit").size(14),
            text("View").size(14),
            text("Tools").size(14),
            text("Help").size(14),
        ]
        .spacing(16)
        .padding(8);

        container(menubar_row)
            .width(Length::Fill)
            .into()
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
    iced::application(Storybook::new, Storybook::update, Storybook::view)
        .title(|state: &Storybook| state.title())
        .run()
}
