# Technical Specification: Project Management System

**Version**: 1.0  
**Created**: 2026-03-15  
**Status**: Draft

## Overview

The Project Management System provides workspace management functionality similar to JetBrains IDEs. Each project represents a story world with its own isolated data, settings, and content stored in a dedicated folder structure.

## Core Concepts

### Project Definition
A **project** is a self-contained story world consisting of:
- A root directory containing all project files
- A `.storybook` folder with metadata and configuration
- Story content, characters, world events, and lore data

### Project Lifecycle
1. **Create**: User creates a new project with a name and location
2. **Load**: User opens an existing project from the project list
3. **Auto-load**: Most recently opened project loads automatically on startup (if valid)
4. **Close**: Project is closed and timestamp is updated
5. **Delete**: Project is removed from the list (files remain on disk)

## Data Structures

### Project Metadata
```rust
struct Project {
    name: String,           // Project name (alphanumeric, hyphens, underscores only)
    path: PathBuf,          // Absolute path to project root directory
    last_opened: DateTime,  // Timestamp of last access
}
```

### Project List Storage
- **Location**: `~/.config/storybook/projects.json`
- **Format**: JSON array of project metadata
- **Schema**:
```json
{
  "projects": [
    {
      "name": "my-fantasy-world",
      "path": "/home/user/storybook/my-fantasy-world",
      "last_opened": "2026-03-15T22:30:00Z"
    }
  ]
}
```

## File System Structure

### Global Configuration
```
~/.config/storybook/
├── projects.json          # List of known projects
└── settings.json          # Global app settings (future)
```

### Project Structure
```
~/storybook/project-name/
├── .storybook/
│   ├── project.json       # Project-specific settings
│   ├── characters.json    # Character database (future)
│   ├── events.json        # World events timeline (future)
│   ├── lore.json          # Lore repository (future)
│   └── metadata.json      # Additional project metadata
└── chapters/              # Story content (future)
```

### `.storybook/project.json` Schema
```json
{
  "name": "my-fantasy-world",
  "created": "2026-03-15T22:00:00Z",
  "version": "0.1.0"
}
```

## Project Naming Rules

### Valid Characters
- Alphanumeric: `a-z`, `A-Z`, `0-9`
- Hyphens: `-`
- Underscores: `_`

### Restrictions
- No spaces
- No special characters (`!@#$%^&*()+=[]{}|;:'",.<>?/\`)
- Minimum length: 1 character
- Maximum length: 64 characters
- Cannot start with `.` (hidden file)

### Validation Regex
```regex
^[a-zA-Z0-9_-]{1,64}$
```

## Project Path Resolution

### Default Path
- **Linux/macOS**: `~/storybook/`
- **Windows**: `%USERPROFILE%\storybook\`

### Path Construction
```
{user_selected_base_path}/{project_name}/
```

**Example**:
- Base path: `/home/user/storybook`
- Project name: `my-fantasy-world`
- Final path: `/home/user/storybook/my-fantasy-world/`

## Project States

### Available
- Project path exists on disk
- `.storybook` folder is present and valid
- Can be loaded normally

### Unavailable
- Project path does not exist (moved/deleted)
- `.storybook` folder is missing or corrupted
- Displayed with visual indicator (grayed out, warning icon)
- User can delete from list but cannot load

## Startup Behavior

### First Launch (No Projects)
1. Show project management view
2. Display "Create New Project" prompt
3. No auto-load attempt

### Subsequent Launches
1. Load `projects.json` from config directory
2. Sort projects by `last_opened` (most recent first)
3. Validate most recent project path
4. **If valid**: Auto-load most recent project
5. **If invalid**: Show project management view with unavailable state

### Fallback to Project Management View
Triggered when:
- No projects exist in `projects.json`
- Most recent project path is invalid
- User explicitly closes current project
- Project load fails for any reason

## User Interface Specification

### Project Management View Layout

```
┌─────────────────────────────────────────────────────────────┐
│  Storybook - Project Management                             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  [+ Create New Project]                                      │
│                                                              │
│  ┌────────────────────────────┐  ┌────────────────────────┐ │
│  │ Project Name          [×]  │  │ Another Project   [×]  │ │
│  │ Last opened: 2h ago        │  │ Last opened: 1d ago    │ │
│  │ /home/user/storybook/...   │  │ /home/user/storybook/  │ │
│  └────────────────────────────┘  └────────────────────────┘ │
│                                                              │
│  ┌────────────────────────────┐  ┌────────────────────────┐ │
│  │ Unavailable Project   [×]  │  │                        │ │
│  │ Last opened: 3d ago   ⚠️   │  │                        │ │
│  │ /invalid/path/...          │  │                        │ │
│  └────────────────────────────┘  └────────────────────────┘ │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Project Card Design

**Available Project Card**:
```
┌────────────────────────────────┐
│ Project Name              [×]  │  ← Name (white) + Delete button
│ Last opened: 2 hours ago       │  ← Timestamp (light gray)
│ /home/user/storybook/proj...   │  ← Path (darker gray, truncated)
└────────────────────────────────┘
```

**Unavailable Project Card**:
```
┌────────────────────────────────┐
│ Project Name         ⚠️   [×]  │  ← Grayed out + Warning icon
│ Last opened: 3 days ago        │  ← Timestamp (darker gray)
│ /invalid/path/project...       │  ← Path (red/error color)
└────────────────────────────────┘
```

### Card Styling
- **Available cards**: White background, hover effect, clickable
- **Unavailable cards**: Gray background, no hover, not clickable
- **Delete button**: Small `(×)` icon, top-right corner
- **Grid layout**: 2-3 cards per row (responsive)
- **Card spacing**: 16px gap between cards
- **Card padding**: 16px internal padding

### Interactions

#### Create New Project
1. Click "Create New Project" button
2. Show dialog/form:
   - Project name input (validated)
   - Path selector (default: `~/storybook/`)
   - Preview: Full path display
   - Create/Cancel buttons
3. On create:
   - Validate name
   - Create directory structure
   - Initialize `.storybook/project.json`
   - Add to `projects.json`
   - Load new project

#### Load Project
1. Click on available project card
2. Validate path still exists
3. Load project data
4. Update `last_opened` timestamp
5. Switch to main application view

#### Delete Project
1. Click delete `(×)` button on card
2. Show confirmation dialog:
   - "Remove '[Project Name]' from list?"
   - "Files will remain on disk"
   - Confirm/Cancel buttons
3. On confirm:
   - Remove from `projects.json`
   - Refresh project list view

## Error Handling

### Project Creation Errors
- **Name validation fails**: Show inline error message
- **Path already exists**: Ask to load existing or choose different name
- **Permission denied**: Show error, suggest different location
- **Disk full**: Show error with disk space info

### Project Loading Errors
- **Path not found**: Mark as unavailable, show in project list
- **Corrupted `.storybook`**: Show error, offer to reinitialize
- **Permission denied**: Show error, mark as unavailable

### Persistence Errors
- **Cannot write `projects.json`**: Show error, continue in-memory
- **Cannot read `projects.json`**: Start with empty project list

## Dependencies

### Rust Crates
- `serde` + `serde_json`: JSON serialization
- `chrono`: Timestamp handling
- `dirs`: Cross-platform directory paths
- `iced`: UI components (cards, buttons, dialogs)

### File I/O
- Standard library `std::fs` for file operations
- `std::path::PathBuf` for path handling

## Future Considerations

### Version 2 Features
- Project templates (fantasy, sci-fi, etc.)
- Project export/import (zip archives)
- Project search/filter in management view
- Recent files list per project
- Project tags/categories
- Cloud sync support
- Project statistics (word count, character count, etc.)

### Migration Strategy
- Version field in `project.json` for future schema changes
- Backward compatibility for older project formats
- Migration utilities for breaking changes

## Security Considerations

- Validate all file paths to prevent directory traversal
- Sanitize project names before filesystem operations
- Handle symbolic links appropriately
- Respect filesystem permissions
- No sensitive data in project metadata (future: encryption support)

## Performance Considerations

- Lazy load project data (don't read all projects on startup)
- Cache project list in memory
- Debounce file system operations
- Async I/O for project loading
- Limit number of displayed projects (pagination for 100+ projects)
