# Task: FEAT1-gui-setup

**Task ID**: FEAT1-gui-setup  
**Created**: 2026-03-15  
**Status**: Pending  
**Branch**: FEAT1-gui-setup

## Objective

Set up the Iced GUI framework dependencies and create a minimal GUI application that opens a window with basic structure.

## Scope

- Add Iced framework dependency to Cargo.toml
- Create basic Iced application structure in main.rs
- Verify the application compiles and launches successfully
- Establish foundation for future GUI components (tabs, panels, writing interface)

## Implementation Steps

### Step 1: Update Dependencies

**File**: `Cargo.toml`

Add Iced dependency:
```toml
[dependencies]
iced = "0.12"
```

### Step 2: Create Basic Application Structure

**File**: `src/main.rs`

Replace the current hello world with a minimal Iced application:
```rust
use iced::{Application, Settings};

struct Storybook;

#[derive(Debug, Clone)]
enum Message {}

impl Application for Storybook {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        (Storybook, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("Storybook")
    }

    fn update(&mut self, _message: Message) -> iced::Command<Message> {
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Message> {
        iced::widget::text("Storybook - Fiction Writing IDE")
            .size(24)
            .into()
    }
}

fn main() -> iced::Result {
    Storybook::run(Settings::default())
}
```

## Testing

### Compile Test
```bash
cargo build
```

### Launch Test
```bash
cargo run
```

**Expected Result**: A window opens with the title "Storybook" and displays centered text "Storybook - Fiction Writing IDE"

## Success Criteria

- [ ] Cargo.toml includes iced dependency
- [ ] Application compiles without errors
- [ ] Application launches and displays a window
- [ ] Window has title "Storybook"
- [ ] Window displays the text content
- [ ] Code follows Iced Application pattern
- [ ] Changes committed to git branch FEAT1-gui-setup

## Notes

- This establishes the foundation for the application
- Future tasks will build upon this structure to add:
  - Tab container workspace
  - Writing pane with markdown support
  - Character database UI
  - World events timeline
  - Lore repository
  - AI assistant panel
  - Project management interface
