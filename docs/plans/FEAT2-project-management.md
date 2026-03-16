# Task: FEAT2-project-management

**Task ID**: FEAT2-project-management  
**Created**: 2026-03-15  
**Status**: Pending  
**Branch**: FEAT2-project-management  
**Depends On**: FEAT1-gui-setup

## Objective

Implement the project management system that allows users to create, load, and manage story world projects. This includes persistent storage of project metadata, automatic loading of the most recent project, and a card-based grid UI for project selection.

## Scope

- Define project data structures (Project, ProjectList)
- Implement JSON persistence for project metadata
- Create project validation and path resolution logic
- Build project management UI with card-based grid layout
- Implement create/load/delete project operations
- Add auto-load functionality for most recent project
- Handle unavailable projects (invalid paths)

## Related Documents

- **Specification**: `docs/specs/project-management.md`

## Implementation Steps

### Step 1: Add Dependencies

**File**: `Cargo.toml`

Add required crates:
```toml
[dependencies]
iced = "0.12"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
dirs = "5.0"
```

### Step 2: Create Project Data Structures

**File**: `src/project.rs` (new file)

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub last_opened: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectList {
    pub projects: Vec<Project>,
}

impl Project {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            last_opened: Utc::now(),
        }
    }

    pub fn is_available(&self) -> bool {
        self.path.exists() && self.path.join(".storybook").exists()
    }

    pub fn validate_name(name: &str) -> bool {
        if name.is_empty() || name.len() > 64 {
            return false;
        }
        name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }
}

impl ProjectList {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        if !config_path.exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(config_path)?;
        let list: ProjectList = serde_json::from_str(&contents)?;
        Ok(list)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, contents)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?;
        Ok(config_dir.join("storybook").join("projects.json"))
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }

    pub fn remove_project(&mut self, path: &PathBuf) {
        self.projects.retain(|p| &p.path != path);
    }

    pub fn update_last_opened(&mut self, path: &PathBuf) {
        if let Some(project) = self.projects.iter_mut().find(|p| &p.path == path) {
            project.last_opened = Utc::now();
        }
    }

    pub fn most_recent(&self) -> Option<&Project> {
        self.projects.iter().max_by_key(|p| p.last_opened)
    }

    pub fn sorted_by_recent(&self) -> Vec<&Project> {
        let mut projects: Vec<&Project> = self.projects.iter().collect();
        projects.sort_by(|a, b| b.last_opened.cmp(&a.last_opened));
        projects
    }
}
```

### Step 3: Create Project Initialization Logic

**File**: `src/project.rs` (add to existing)

```rust
impl Project {
    pub fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create project directory
        std::fs::create_dir_all(&self.path)?;
        
        // Create .storybook directory
        let storybook_dir = self.path.join(".storybook");
        std::fs::create_dir_all(&storybook_dir)?;
        
        // Create project.json
        let project_config = serde_json::json!({
            "name": self.name,
            "created": Utc::now(),
            "version": "0.1.0"
        });
        let config_path = storybook_dir.join("project.json");
        std::fs::write(config_path, serde_json::to_string_pretty(&project_config)?)?;
        
        Ok(())
    }

    pub fn default_base_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("storybook")
    }
}
```

### Step 4: Update Application State

**File**: `src/main.rs`

Update to include project management state:
```rust
use iced::{Application, Settings, Command};
mod project;
use project::{Project, ProjectList};

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
            |result| Message::ProjectsLoaded(result.map_err(|e| e.to_string()))
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
                // Auto-load most recent project if available
                if let Some(recent) = self.project_list.most_recent() {
                    if recent.is_available() {
                        let path = recent.path.clone();
                        return Command::perform(
                            async move { Ok(recent.clone()) },
                            Message::ProjectLoaded
                        );
                    }
                }
                Command::none()
            }
            Message::ProjectsLoaded(Err(_)) => {
                // Start with empty project list
                Command::none()
            }
            Message::CreateProject(name, base_path) => {
                let path = base_path.join(&name);
                let project = Project::new(name, path);
                Command::perform(
                    async move {
                        project.initialize()?;
                        Ok(project)
                    },
                    |result| Message::ProjectCreated(result.map_err(|e: Box<dyn std::error::Error>| e.to_string()))
                )
            }
            Message::ProjectCreated(Ok(project)) => {
                self.project_list.add_project(project.clone());
                let _ = self.project_list.save();
                self.current_project = Some(project);
                self.view = AppView::MainWorkspace;
                Command::none()
            }
            Message::ProjectCreated(Err(_)) => {
                // Show error (future: error state in UI)
                Command::none()
            }
            Message::LoadProject(path) => {
                if let Some(project) = self.project_list.projects.iter().find(|p| p.path == path) {
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
            Message::ProjectLoaded(Err(_)) => {
                // Failed to auto-load, stay on project management
                Command::none()
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
        // Placeholder for now
        iced::widget::text("Project Management View - Coming Soon")
            .size(24)
            .into()
    }

    fn view_main_workspace(&self) -> iced::Element<Message> {
        if let Some(project) = &self.current_project {
            iced::widget::text(format!("Workspace: {}", project.name))
                .size(24)
                .into()
        } else {
            iced::widget::text("No project loaded")
                .size(24)
                .into()
        }
    }
}

fn main() -> iced::Result {
    Storybook::run(Settings::default())
}
```

### Step 5: Implement Project Management UI

**File**: `src/main.rs` (update `view_project_management` method)

```rust
use iced::widget::{button, column, container, row, text, scrollable, Space};
use iced::{Alignment, Length};

impl Storybook {
    fn view_project_management(&self) -> iced::Element<Message> {
        let title = text("Storybook - Project Management")
            .size(32);

        let create_button = button(text("+ Create New Project"))
            .on_press(Message::ShowCreateDialog); // Future: dialog

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
        let sorted_projects = self.project_list.sorted_by_recent();
        
        // Create rows of project cards (2 per row)
        let mut rows_vec = Vec::new();
        let mut current_row = Vec::new();
        
        for (i, project) in sorted_projects.iter().enumerate() {
            current_row.push(self.view_project_card(project));
            
            if (i + 1) % 2 == 0 || i == sorted_projects.len() - 1 {
                rows_vec.push(
                    row(current_row.clone())
                        .spacing(16)
                        .into()
                );
                current_row.clear();
            }
        }

        column(rows_vec)
            .spacing(16)
            .into()
    }

    fn view_project_card(&self, project: &Project) -> iced::Element<Message> {
        let is_available = project.is_available();
        
        let name_text = text(&project.name)
            .size(18);
        
        let time_ago = format_time_ago(&project.last_opened);
        let timestamp_text = text(format!("Last opened: {}", time_ago))
            .size(14);
        
        let path_text = text(project.path.display().to_string())
            .size(12);
        
        let delete_button = button(text("×").size(20))
            .on_press(Message::DeleteProject(project.path.clone()));
        
        let header = row![
            name_text,
            Space::with_width(Length::Fill),
            if !is_available { 
                text("⚠️").size(16).into() 
            } else { 
                Space::with_width(0).into() 
            },
            delete_button,
        ]
        .align_items(Alignment::Center);
        
        let card_content = column![
            header,
            timestamp_text,
            path_text,
        ]
        .spacing(8)
        .padding(16);
        
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
```

### Step 6: Add Module Declaration

**File**: `src/main.rs` (at top)

```rust
mod project;
```

## Testing

### Manual Testing Steps

1. **First Launch**
   ```bash
   cargo run
   ```
   - Should show project management view
   - No projects displayed

2. **Create Project** (future: via UI, for now test programmatically)
   - Verify project directory created
   - Verify `.storybook/project.json` exists
   - Verify added to `~/.config/storybook/projects.json`

3. **Load Project**
   - Click on project card
   - Verify workspace view loads
   - Verify window title updates

4. **Delete Project**
   - Click delete button
   - Verify project removed from list
   - Verify files remain on disk

5. **Auto-load**
   - Close and restart app
   - Verify most recent project loads automatically

6. **Unavailable Project**
   - Delete a project folder from disk
   - Restart app
   - Verify project shows as unavailable
   - Verify cannot be loaded
   - Verify can be deleted from list

### Unit Tests

**File**: `src/project.rs` (add at bottom)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name_valid() {
        assert!(Project::validate_name("my-project"));
        assert!(Project::validate_name("my_project"));
        assert!(Project::validate_name("MyProject123"));
    }

    #[test]
    fn test_validate_name_invalid() {
        assert!(!Project::validate_name("my project")); // space
        assert!(!Project::validate_name("my@project")); // special char
        assert!(!Project::validate_name("")); // empty
        assert!(!Project::validate_name(&"a".repeat(65))); // too long
    }

    #[test]
    fn test_project_list_sorted() {
        let mut list = ProjectList::default();
        let p1 = Project::new("old".to_string(), PathBuf::from("/old"));
        let mut p2 = Project::new("new".to_string(), PathBuf::from("/new"));
        p2.last_opened = chrono::Utc::now();
        
        list.add_project(p1);
        list.add_project(p2);
        
        let sorted = list.sorted_by_recent();
        assert_eq!(sorted[0].name, "new");
        assert_eq!(sorted[1].name, "old");
    }
}
```

## Success Criteria

- [ ] Dependencies added to Cargo.toml
- [ ] Project data structures implemented with serialization
- [ ] Project validation logic working
- [ ] Project initialization creates proper directory structure
- [ ] ProjectList persistence (load/save) working
- [ ] Project management UI displays card grid
- [ ] Create project functionality working
- [ ] Load project functionality working
- [ ] Delete project functionality working
- [ ] Auto-load most recent project working
- [ ] Unavailable projects displayed correctly
- [ ] Unit tests passing
- [ ] Application compiles and runs
- [ ] Changes committed to git branch FEAT2-project-management

## Notes

- This task establishes project workspace management
- Future tasks will add:
  - Create project dialog with name validation
  - Confirmation dialog for delete
  - Better error handling and user feedback
  - Project search/filter
  - Character database per project
  - World events timeline per project
  - Lore repository per project

## Future Enhancements

- Project templates
- Import/export projects
- Project statistics
- Cloud sync
- Collaborative editing
