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
