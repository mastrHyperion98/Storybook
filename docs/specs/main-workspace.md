# Technical Specification: Main Workspace Layout

**Version**: 1.0  
**Created**: 2026-03-15  
**Status**: Draft

## Overview

The main workspace provides the primary interface for working with a story project. It features a JetBrains-style IDE layout with a menubar, tab container system, and multiple tool panels that can be arranged side-by-side.

## Design Philosophy

- **IDE-inspired**: Layout follows JetBrains IDE conventions (IntelliJ, PyCharm, etc.)
- **Tab-based workflow**: Multiple tools/views open simultaneously in tabs
- **Flexible arrangement**: Users can move and resize panels
- **Writing-focused**: Writing pane is the default and primary view
- **Scalable**: Supports 720p to ultrawide 5220x1440p displays

## Core Layout Structure

```
┌─────────────────────────────────────────────────────────────┐
│ File   Edit   View   Tools   Help                          │ ← Menubar
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────────────┐ ┌─────────────────────────────────┐ │
│ │ Tab: Writing        │ │ Tab: Characters                 │ │
│ ├─────────────────────┤ ├─────────────────────────────────┤ │
│ │                     │ │                                 │ │
│ │  Writing Pane       │ │  Character Database             │ │
│ │  (Markdown)         │ │                                 │ │
│ │                     │ │                                 │ │
│ │                     │ │                                 │ │
│ └─────────────────────┘ └─────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Menubar Specification

### Menu Structure

**File Menu**
- New Chapter
- Save (Ctrl+S)
- Save All (Ctrl+Shift+S)
- ---
- Close Project (returns to project management)
- Exit Application (Ctrl+Q)

**Edit Menu** (Future)
- Undo (Ctrl+Z)
- Redo (Ctrl+Shift+Z)
- ---
- Cut (Ctrl+X)
- Copy (Ctrl+C)
- Paste (Ctrl+V)

**View Menu** (Future)
- Show Writing Pane
- Show Character Database
- Show World Events
- Show Lore Repository
- Show AI Assistant
- ---
- Toggle Dark Mode

**Tools Menu** (Future)
- AI Assistant
- Generate Character
- Generate Event
- Export Project

**Help Menu** (Future)
- Documentation
- Keyboard Shortcuts
- About

### Menubar Behavior

**Close Project**:
1. User selects "File > Close Project"
2. Save any unsaved changes (future: prompt if needed)
3. Clear current project state
4. Return to project management view
5. Do NOT update `last_opened` timestamp (project was closed, not loaded)

**Exit Application**:
1. User selects "File > Exit Application" or presses Ctrl+Q
2. Save any unsaved changes (future: prompt if needed)
3. Update `last_opened` timestamp for current project
4. Exit application

## Tab Container System

### Tab Types

1. **Writing Tab** (default)
   - Markdown editor
   - Chapter/section navigation
   - Word count display

2. **Character Database Tab**
   - List of characters
   - Character detail view
   - Family tree visualization

3. **World Events Tab**
   - Timeline view
   - Event list
   - Event detail editor

4. **Lore Repository Tab**
   - Document list
   - Document viewer/editor

5. **AI Assistant Tab**
   - Mode selector (Chat/Instruct/Edit)
   - Conversation interface
   - Action buttons

### Tab Behavior

- **Default**: Writing tab opens automatically when project loads
- **Multiple tabs**: Users can open multiple tabs simultaneously
- **Side-by-side**: Tabs can be arranged in split view (horizontal/vertical)
- **Drag and drop**: Tabs can be reordered and moved between splits
- **Close**: Individual tabs can be closed (except at least one must remain)

## Layout Modes

### Single Panel (Default)
```
┌─────────────────────────────────────┐
│ Menubar                             │
├─────────────────────────────────────┤
│ Tab: Writing                        │
├─────────────────────────────────────┤
│                                     │
│         Content Area                │
│                                     │
└─────────────────────────────────────┘
```

### Vertical Split
```
┌─────────────────────────────────────┐
│ Menubar                             │
├──────────────────┬──────────────────┤
│ Tab: Writing     │ Tab: Characters  │
├──────────────────┼──────────────────┤
│                  │                  │
│   Writing Pane   │  Character DB    │
│                  │                  │
└──────────────────┴──────────────────┘
```

### Horizontal Split
```
┌─────────────────────────────────────┐
│ Menubar                             │
├─────────────────────────────────────┤
│ Tab: Writing                        │
├─────────────────────────────────────┤
│         Writing Pane                │
├─────────────────────────────────────┤
│ Tab: AI Assistant                   │
├─────────────────────────────────────┤
│         AI Chat                     │
└─────────────────────────────────────┘
```

## State Management

### Workspace State
```rust
struct WorkspaceState {
    current_project: Project,
    open_tabs: Vec<Tab>,
    layout: LayoutConfig,
    active_tab: usize,
}

enum Tab {
    Writing(WritingState),
    Characters(CharacterState),
    WorldEvents(EventState),
    Lore(LoreState),
    AIAssistant(AIState),
}

struct LayoutConfig {
    splits: Vec<Split>,
    split_ratios: Vec<f32>,
}
```

### Message Types
```rust
enum Message {
    // Menubar actions
    CloseProject,
    ExitApplication,
    
    // Tab management
    OpenTab(TabType),
    CloseTab(usize),
    SwitchTab(usize),
    
    // Layout
    SplitVertical,
    SplitHorizontal,
    CloseSplit,
    ResizeSplit(f32),
}
```

## Initial Implementation (FEAT3)

For this feature, we implement:

1. **Menubar**
   - File menu with "Close Project" option
   - Basic menu structure (other items disabled/placeholder)

2. **Basic Layout**
   - Single panel view
   - Placeholder content area showing project name

3. **Close Project Functionality**
   - Clear current project state
   - Return to project management view
   - Maintain project list without updating timestamp

4. **Workspace View**
   - Replace current placeholder with proper layout
   - Show menubar at top
   - Show content area below

## UI Components

### Menubar Component
- Horizontal bar at top of window
- Menu items: File, Edit, View, Tools, Help
- Dropdown menus on click
- Keyboard shortcuts displayed
- Hover effects

### Content Area
- Fills remaining space below menubar
- Contains tab container
- Responsive to window resize

### Tab Bar
- Horizontal tabs at top of content area
- Active tab highlighted
- Close button (×) on each tab
- Add tab button (+) on right

## Styling

### Menubar
- Height: 30px
- Background: Dark gray (#2b2b2b)
- Text: Light gray (#cccccc)
- Hover: Lighter background (#3c3c3c)
- Active: Blue highlight (#0d7377)

### Tabs
- Height: 35px
- Background: Dark gray (#2b2b2b)
- Active tab: Darker background (#1e1e1e)
- Text: Light gray (#cccccc)
- Border: 1px subtle separator

### Content Area
- Background: Dark (#1e1e1e)
- Text: Light (#d4d4d4)
- Padding: 20px

## Keyboard Shortcuts

- **Ctrl+W**: Close current tab
- **Ctrl+Shift+W**: Close project
- **Ctrl+Q**: Exit application
- **Ctrl+Tab**: Next tab
- **Ctrl+Shift+Tab**: Previous tab
- **Ctrl+1-9**: Switch to tab 1-9

## Responsive Behavior

### Small Screens (720p - 1280x720)
- Single panel mode recommended
- Menubar text may abbreviate
- Minimum window width: 800px
- Minimum window height: 600px

### Medium Screens (1080p - 1920x1080)
- Comfortable for 2-panel split
- Default layout works well

### Large Screens (1440p+ and Ultrawide)
- Support 3+ panel splits
- More horizontal space for side-by-side views
- Wider content areas

## Future Enhancements

### Phase 2
- Drag and drop tab reordering
- Split view implementation
- Resizable panels with draggable dividers
- Tab persistence (remember open tabs per project)

### Phase 3
- Detachable tabs (floating windows)
- Custom layouts saved per project
- Panel pinning/unpinning
- Minimap for writing pane

## Implementation Notes

### Iced Limitations
- Iced doesn't have built-in menubar widget
- Need to create custom menubar using buttons/containers
- Split view requires manual layout calculation
- Tab dragging requires custom event handling

### Workarounds
- Use row of buttons for menubar
- Use column/row containers for splits
- Store layout state in application struct
- Handle tab switching via message passing

## Testing Criteria

- [ ] Menubar displays correctly
- [ ] File menu opens on click
- [ ] Close Project returns to project management
- [ ] Project state is cleared on close
- [ ] Layout scales to different window sizes
- [ ] Keyboard shortcuts work
- [ ] No crashes or panics
