use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length};

use crate::project::{Project, ProjectList};
use crate::Message;

pub fn view_project_management<'a>(
    project_list: &'a ProjectList,
    show_create_dialog: bool,
    new_project_name: &'a str,
    new_project_path: &'a str,
    create_error: &'a Option<String>,
) -> Element<'a, Message> {
    let title = text("Storybook - Project Management").size(32);

    let create_button = button(text("+ Create New Project"))
        .padding(10)
        .on_press(Message::ShowCreateDialog);

    let projects_grid = view_projects_grid(project_list);

    let content = column![
        title,
        Space::new().height(20),
        create_button,
        Space::new().height(30),
        projects_grid,
    ]
    .padding(40)
    .align_x(Alignment::Start);

    let base_view = container(scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill);

    if show_create_dialog {
        view_create_dialog_overlay(
            base_view.into(),
            new_project_name,
            new_project_path,
            create_error,
        )
    } else {
        base_view.into()
    }
}

fn view_create_dialog_overlay<'a>(
    _base: Element<'a, Message>,
    new_project_name: &'a str,
    new_project_path: &'a str,
    create_error: &'a Option<String>,
) -> Element<'a, Message> {
    let dialog_title = text("Create New Project").size(24);

    let name_label = text("Project Name:").size(14);
    let name_input = text_input(
        "my-project",
        new_project_name,
    )
    .on_input(Message::ProjectNameChanged)
    .padding(10);

    let path_label = text("Base Path:").size(14);
    let path_input = text_input(
        "/home/user/storybook",
        new_project_path,
    )
    .on_input(Message::ProjectPathChanged)
    .padding(10);

    let preview_path = if !new_project_name.is_empty() {
        format!(
            "Full path: {}/{}",
            new_project_path, new_project_name
        )
    } else {
        "Full path: (enter project name)".to_string()
    };
    let preview_text = text(preview_path).size(12);

    let error_message = if let Some(err) = create_error {
        column![
            text(err).size(14),
            Space::new().height(10),
        ]
    } else {
        column![]
    };

    let create_btn = button(text("Create"))
        .padding(10)
        .on_press(Message::CreateProject);

    let cancel_btn = button(text("Cancel"))
        .padding(10)
        .on_press(Message::HideCreateDialog);

    let buttons = row![create_btn, Space::new().width(10), cancel_btn]
        .spacing(10);

    let dialog_content = column![
        dialog_title,
        Space::new().height(20),
        name_label,
        name_input,
        Space::new().height(15),
        path_label,
        path_input,
        Space::new().height(10),
        preview_text,
        Space::new().height(20),
        error_message,
        buttons,
    ]
    .padding(30)
    .align_x(Alignment::Start)
    .width(Length::Fixed(500.0));

    let dialog = container(dialog_content)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fill);

    dialog.into()
}

fn view_projects_grid<'a>(project_list: &'a ProjectList) -> Element<'a, Message> {
    let sorted_projects = project_list.sorted_by_recent();

    let mut rows_vec = Vec::new();
    let mut current_row = Vec::new();

    for (i, project) in sorted_projects.iter().enumerate() {
        current_row.push(view_project_card(project));

        if (i + 1) % 2 == 0 || i == sorted_projects.len() - 1 {
            let row_elements = current_row.drain(..).collect::<Vec<_>>();
            rows_vec.push(row(row_elements).spacing(16).into());
        }
    }

    column(rows_vec).spacing(16).into()
}

fn view_project_card<'a>(project: &'a Project) -> Element<'a, Message> {
    let is_available = project.is_available();

    let name_text = text(&project.name).size(18);

    let time_ago = crate::format_time_ago(&project.last_opened);
    let timestamp_text = text(format!("Last opened: {}", time_ago)).size(14);

    let path_text = text(project.path.display().to_string()).size(12);

    let delete_button = button(text("×").size(20))
        .on_press(Message::DeleteProject(project.path.clone()));

    let mut header_items = vec![
        name_text.into(),
        Space::new().width(Length::Fill).into(),
    ];
    
    if !is_available {
        header_items.push(text("⚠️").size(16).into());
    }
    
    header_items.push(delete_button.into());
    
    let header = row(header_items).align_y(Alignment::Center);

    let card_content =
        column![header, timestamp_text, path_text,].spacing(8).padding(16);

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
