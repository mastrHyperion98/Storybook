# Task: FEAT3-main-workspace

**Task ID**: FEAT3-main-workspace  
**Created**: 2026-03-15  
**Status**: Completed  
**Branch**: FEAT3-main-workspace  
**Depends On**: FEAT2-project-management

## Objective

Implement the main workspace layout with a menubar and basic IDE-style structure. Focus on creating a menubar with "Close Project" functionality that returns the user to the project management view.

## Scope

- Create menubar component with File menu
- Implement "Close Project" action
- Update workspace view with proper layout structure
- Add message handling for closing projects
- Ensure project state is cleared when closing
- Verify return to project management view works correctly

## Related Documents

- **Specification**: `docs/specs/main-workspace.md`

## Implementation Steps

### Step 1: Add Message Types for Workspace Actions

**File**: `src/main.rs`

Add new message variants:
```rust
#[derive(Debug, Clone)]
enum Message {
    // ... existing messages
    CloseProject,
    ToggleFileMenu,
}
```

Add menubar state to Storybook struct:
```rust
struct Storybook {
    // ... existing fields
    show_file_menu: bool,
}
```

### Step 2: Implement Close Project Logic

**File**: `src/main.rs` (update method)

```rust
fn update(&mut self, message: Message) -> Command<Message> {
    match message {
        // ... existing message handlers
        
        Message::ToggleFileMenu => {
            self.show_file_menu = !self.show_file_menu;
            Command::none()
        }
        
        Message::CloseProject => {
            // Clear current project without updating timestamp
            self.current_project = None;
            self.view = AppView::ProjectManagement;
            self.show_file_menu = false;
            Command::none()
        }
    }
}
```

### Step 3: Create Menubar Component

**File**: `src/main.rs`

Add menubar view method:
```rust
impl Storybook {
    fn view_menubar(&self) -> iced::Element<Message> {
        use iced::widget::{button, column, container, row, text};
        use iced::{Alignment, Length};

        let file_button = button(text("File"))
            .on_press(Message::ToggleFileMenu)
            .padding(8);

        let edit_button = button(text("Edit")).padding(8);
        let view_button = button(text("View")).padding(8);
        let tools_button = button(text("Tools")).padding(8);
        let help_button = button(text("Help")).padding(8);

        let menubar = row![
            file_button,
            edit_button,
            view_button,
            tools_button,
            help_button,
        ]
        .spacing(5)
        .padding(5);

        container(menubar)
            .width(Length::Fill)
            .into()
    }

    fn view_file_menu(&self) -> Option<iced::Element<Message>> {
        if !self.show_file_menu {
            return None;
        }

        use iced::widget::{button, column, container, text};
        use iced::Length;

        let close_project = button(text("Close Project"))
            .on_press(Message::CloseProject)
            .width(Length::Fill)
            .padding(8);

        let menu_content = column![close_project]
            .spacing(2)
            .padding(5);

        let menu = container(menu_content)
            .width(Length::Fixed(200.0));

        Some(menu.into())
    }
}
```

### Step 4: Update Main Workspace View

**File**: `src/main.rs`

Update `view_main_workspace` to include menubar:
```rust
fn view_main_workspace(&self) -> iced::Element<Message> {
    use iced::widget::{column, container, text};
    use iced::Length;

    let menubar = self.view_menubar();

    let content = if let Some(project) = &self.current_project {
        column![
            text(format!("Project: {}", project.name)).size(24),
            text("Main workspace content area").size(16),
        ]
        .padding(20)
        .spacing(10)
    } else {
        column![text("No project loaded").size(24)]
            .padding(20)
    };

    let main_content = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

    let mut layout = column![menubar, main_content]
        .width(Length::Fill)
        .height(Length::Fill);

    // Overlay file menu if open
    if let Some(file_menu) = self.view_file_menu() {
        // Position menu below File button
        // For now, just show it (proper positioning in future)
        layout = column![menubar, file_menu, main_content]
            .width(Length::Fill)
            .height(Length::Fill);
    }

    layout.into()
}
```

### Step 5: Initialize New State Fields

**File**: `src/main.rs`

Update `new()` method:
```rust
fn new(_flags: ()) -> (Self, Command<Message>) {
    // ... existing code
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
            show_file_menu: false,  // Add this
        },
        cmd,
    )
}
```

### Step 6: Handle Click Outside to Close Menu

**File**: `src/main.rs`

Add message for clicking outside menu:
```rust
#[derive(Debug, Clone)]
enum Message {
    // ... existing
    CloseMenus,
}
```

Update handler:
```rust
Message::CloseMenus => {
    self.show_file_menu = false;
    Command::none()
}
```

## Testing

### Manual Testing Steps

1. **Launch Application**
   ```bash
   cargo run
   ```

2. **Create or Load Project**
   - Create a new project or load existing
   - Verify workspace view appears

3. **Test Menubar**
   - Verify menubar displays at top
   - Click "File" button
   - Verify dropdown menu appears

4. **Test Close Project**
   - Click "Close Project" in File menu
   - Verify return to project management view
   - Verify project list still shows projects
   - Verify no timestamp update occurred

5. **Test Re-load Project**
   - Load the same project again
   - Verify it loads successfully
   - Verify workspace appears again

6. **Test Menu Closing**
   - Open File menu
   - Click elsewhere
   - Verify menu closes (future enhancement)

### Unit Tests

**File**: `src/main.rs` (add at bottom)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_close_project_clears_state() {
        let mut app = Storybook {
            view: AppView::MainWorkspace,
            project_list: ProjectList::default(),
            current_project: Some(Project::new(
                "test".to_string(),
                PathBuf::from("/test"),
            )),
            show_create_dialog: false,
            new_project_name: String::new(),
            new_project_path: String::new(),
            create_error: None,
            show_file_menu: false,
        };

        app.update(Message::CloseProject);

        assert!(app.current_project.is_none());
        assert!(matches!(app.view, AppView::ProjectManagement));
        assert!(!app.show_file_menu);
    }

    #[test]
    fn test_toggle_file_menu() {
        let mut app = Storybook {
            view: AppView::MainWorkspace,
            project_list: ProjectList::default(),
            current_project: None,
            show_create_dialog: false,
            new_project_name: String::new(),
            new_project_path: String::new(),
            create_error: None,
            show_file_menu: false,
        };

        app.update(Message::ToggleFileMenu);
        assert!(app.show_file_menu);

        app.update(Message::ToggleFileMenu);
        assert!(!app.show_file_menu);
    }
}
```

## Success Criteria

- [ ] Menubar displays at top of workspace
- [ ] File menu button toggles dropdown
- [ ] Close Project option appears in File menu
- [ ] Clicking Close Project returns to project management
- [ ] Current project state is cleared
- [ ] Project list remains intact
- [ ] No timestamp update on close
- [ ] Can re-load project after closing
- [ ] Application compiles without errors
- [ ] Unit tests pass
- [ ] Manual testing successful

## Notes

- This is a minimal implementation focusing on core functionality
- Future enhancements will add:
  - More menu items (Edit, View, Tools, Help)
  - Keyboard shortcuts (Ctrl+W, Ctrl+Q)
  - Proper menu positioning and styling
  - Click-outside-to-close behavior
  - Tab container system
  - Split view layouts

## Future Enhancements

### Phase 2: Enhanced Menubar
- Add all menu categories
- Implement keyboard shortcuts
- Add menu item icons
- Improve styling and hover effects
- Click-outside-to-close

### Phase 3: Tab System
- Tab bar component
- Multiple tabs support
- Tab switching
- Tab close buttons

### Phase 4: Layout System
- Split view (vertical/horizontal)
- Resizable panels
- Drag and drop tabs
- Layout persistence
