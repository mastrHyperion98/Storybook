use iced::Task;

mod project;
mod components;
mod views;

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
    sidebar_collapsed: bool,
    selected_secondary_panel: SecondaryPanel,
}

enum AppView {
    ProjectManagement,
    MainWorkspace,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SecondaryPanel {
    CharacterDatabase,
    WorldEvents,
    Lore,
    AiAssistant,
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
    ToggleSidebarCollapse,
    SelectSecondaryPanel(SecondaryPanel),
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
                sidebar_collapsed: false,
                selected_secondary_panel: SecondaryPanel::CharacterDatabase,
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
            Message::ToggleSidebarCollapse => {
                self.sidebar_collapsed = !self.sidebar_collapsed;
                Task::none()
            }
            Message::SelectSecondaryPanel(panel) => {
                self.selected_secondary_panel = panel;
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
        views::project_management::view_project_management(
            &self.project_list,
            self.show_create_dialog,
            &self.new_project_name,
            &self.new_project_path,
            &self.create_error,
        )
    }

    fn view_main_workspace(&self) -> iced::Element<Message> {
        use iced::widget::{column, container, row, text, Space};
        use iced::{Border, Length};

        let menubar = components::menubar::view_menubar();

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

        // Create workspace layout: sidebar + writing pane + secondary panel
        let sidebar = components::sidebar::view_sidebar(
            self.sidebar_collapsed,
            self.selected_secondary_panel,
        );
        let writing_pane = views::writing_pane::view_writing_pane();
        let secondary_panel = views::secondary_panel::view_secondary_panel(
            self.selected_secondary_panel,
        );
        
        let workspace_content: iced::Element<Message> = row![
            sidebar,
            writing_pane,
            secondary_panel,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

        let layout = column![menubar, separator, workspace_content]
            .width(Length::Fill)
            .height(Length::Fill);

        layout.into()
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
