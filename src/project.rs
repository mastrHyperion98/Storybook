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
        name.chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }

    pub fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all(&self.path)?;

        let storybook_dir = self.path.join(".storybook");
        std::fs::create_dir_all(&storybook_dir)?;

        let project_config = serde_json::json!({
            "name": self.name,
            "created": Utc::now(),
            "version": "0.1.0"
        });
        let config_path = storybook_dir.join("project.json");
        std::fs::write(
            config_path,
            serde_json::to_string_pretty(&project_config)?,
        )?;

        Ok(())
    }

    pub fn default_base_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("storybook")
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
        let config_dir =
            dirs::config_dir().ok_or("Could not find config directory")?;
        Ok(config_dir.join("storybook").join("projects.json"))
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }

    pub fn remove_project(&mut self, path: &PathBuf) {
        self.projects.retain(|p| &p.path != path);
    }

    pub fn update_last_opened(&mut self, path: &PathBuf) {
        if let Some(project) = self.projects.iter_mut().find(|p| &p.path == path)
        {
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
        assert!(!Project::validate_name("my project"));
        assert!(!Project::validate_name("my@project"));
        assert!(!Project::validate_name(""));
        assert!(!Project::validate_name(&"a".repeat(65)));
    }

    #[test]
    fn test_project_list_sorted() {
        use std::thread;
        use std::time::Duration;

        let mut list = ProjectList::default();
        let p1 = Project::new("old".to_string(), PathBuf::from("/old"));
        list.add_project(p1);

        thread::sleep(Duration::from_millis(10));

        let p2 = Project::new("new".to_string(), PathBuf::from("/new"));
        list.add_project(p2);

        let sorted = list.sorted_by_recent();
        assert_eq!(sorted[0].name, "new");
        assert_eq!(sorted[1].name, "old");
    }
}
