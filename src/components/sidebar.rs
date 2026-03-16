use iced::widget::{button, column, container, text, Space};
use iced::{Background, Border, Color, Element, Length};

use crate::{Message, SecondaryPanel};

pub fn view_sidebar(sidebar_collapsed: bool, selected_secondary_panel: SecondaryPanel) -> Element<'static, Message> {
    if sidebar_collapsed {
        // Collapsed sidebar - just show collapse/expand button
        let expand_btn = button(text("▶").size(12))
            .padding(8)
            .on_press(Message::ToggleSidebarCollapse)
            .style(|_theme, status| {
                let bg_color = match status {
                    iced::widget::button::Status::Hovered => Color::from_rgb(0.25, 0.25, 0.25),
                    _ => Color::from_rgb(0.2, 0.2, 0.2),
                };
                iced::widget::button::Style {
                    background: Some(Background::Color(bg_color)),
                    text_color: Color::from_rgb(0.9, 0.9, 0.9),
                    border: Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: false,
                }
            });
        
        return container(column![expand_btn])
            .width(Length::Fixed(40.0))
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.15))),
                border: Border {
                    color: Color::from_rgb(0.3, 0.3, 0.3),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into();
    }

    // Helper to create tab button
    let create_tab_button = |label: &'static str, panel: SecondaryPanel| {
        let is_active = selected_secondary_panel == panel;
        button(text(label).size(13).width(Length::Fill))
            .width(Length::Fill)
            .padding([10, 12])
            .on_press(Message::SelectSecondaryPanel(panel))
            .style(move |_theme, status| {
                let base_color = if is_active {
                    Color::from_rgb(0.25, 0.25, 0.25)
                } else {
                    Color::TRANSPARENT
                };
                
                let bg_color = match status {
                    iced::widget::button::Status::Hovered => {
                        if is_active {
                            Color::from_rgb(0.28, 0.28, 0.28)
                        } else {
                            Color::from_rgb(0.22, 0.22, 0.22)
                        }
                    }
                    iced::widget::button::Status::Pressed => {
                        Color::from_rgb(0.20, 0.20, 0.20)
                    }
                    _ => base_color,
                };
                
                iced::widget::button::Style {
                    background: Some(Background::Color(bg_color)),
                    text_color: if is_active {
                        Color::from_rgb(1.0, 1.0, 1.0)
                    } else {
                        Color::from_rgb(0.8, 0.8, 0.8)
                    },
                    border: Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: false,
                }
            })
    };

    let collapse_btn = button(text("◀").size(12))
        .padding(6)
        .on_press(Message::ToggleSidebarCollapse)
        .style(|_theme, status| {
            let bg_color = match status {
                iced::widget::button::Status::Hovered => Color::from_rgb(0.25, 0.25, 0.25),
                _ => Color::TRANSPARENT,
            };
            iced::widget::button::Style {
                background: Some(Background::Color(bg_color)),
                text_color: Color::from_rgb(0.7, 0.7, 0.7),
                border: Border::default(),
                shadow: iced::Shadow::default(),
                snap: false,
            }
        });

    let sidebar_content = column![
        container({
            use iced::widget::row;
            row![
                text("Panels").size(13),
                Space::new().width(Length::Fill),
                collapse_btn,
            ]
        })
        .padding([8, 12])
        .width(Length::Fill),
        Space::new().height(4),
        create_tab_button("Characters", SecondaryPanel::CharacterDatabase),
        create_tab_button("World Events", SecondaryPanel::WorldEvents),
        create_tab_button("Lore", SecondaryPanel::Lore),
        create_tab_button("AI Assistant", SecondaryPanel::AiAssistant),
    ]
    .spacing(0);

    container(sidebar_content)
        .width(Length::Fixed(180.0))
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.15))),
            border: Border {
                color: Color::from_rgb(0.3, 0.3, 0.3),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}
