use iced::widget::{container, text};
use iced::{Background, Border, Color, Element, Length};

use crate::{Message, SecondaryPanel};

pub fn view_secondary_panel(selected_panel: SecondaryPanel) -> Element<'static, Message> {
    let panel_name = match selected_panel {
        SecondaryPanel::CharacterDatabase => "Character Database",
        SecondaryPanel::WorldEvents => "World Events",
        SecondaryPanel::Lore => "Lore Repository",
        SecondaryPanel::AiAssistant => "AI Assistant",
    };

    container(
        text(format!("{}\n(Secondary Panel)", panel_name))
            .size(16)
    )
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.12, 0.12, 0.12))),
        border: Border {
            color: Color::from_rgb(0.3, 0.3, 0.3),
            width: 1.0,
            radius: 0.0.into(),
        },
        text_color: Some(Color::from_rgb(0.9, 0.9, 0.9)),
        shadow: iced::Shadow::default(),
        snap: false,
    })
    .into()
}
