use iced::widget::{button, container, row, text};
use iced::{Alignment, Element, Length};
use iced_aw::{menu_bar, menu_items};
use iced_aw::menu::Menu;

use crate::Message;

pub fn view_menubar() -> Element<'static, Message> {
    let close_project_btn = button(
        text("Close Project")
            .width(Length::Fill)
            .align_y(Alignment::Center)
    )
    .width(Length::Fill)
    .on_press(Message::CloseProject);

    let file_menu = Menu::new(menu_items!(
        (close_project_btn)
    ))
    .width(180.0)
    .offset(0.0)
    .spacing(0.0);

    let toggle_sidebar_btn = button(
        text("Toggle Sidebar")
            .width(Length::Fill)
            .align_y(Alignment::Center)
    )
    .width(Length::Fill)
    .on_press(Message::ToggleSidebarCollapse);

    let view_menu = Menu::new(menu_items!(
        (toggle_sidebar_btn)
    ))
    .width(180.0)
    .offset(0.0)
    .spacing(0.0);

    let file_mb = menu_bar!(
        (text("File"), file_menu)
    );

    let view_mb = menu_bar!(
        (text("View"), view_menu)
    );

    let menubar_row = row![
        file_mb,
        view_mb,
        text("Edit").size(14),
        text("Tools").size(14),
        text("Help").size(14),
    ]
    .spacing(16)
    .padding(8);

    container(menubar_row)
        .width(Length::Fill)
        .into()
}
