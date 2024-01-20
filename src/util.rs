use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, ListItem, Paragraph};
use ratatui::Frame;
use tui_textarea::TextArea;
use vcard_parser::constants::PropertyName;
use vcard_parser::traits::{HasName, HasValue};
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::value::Value::ValueListComponent;

use crate::state::popup::PopupState;
use crate::{PRIMARY, PRIMARY_CONTRAST, PRIMARY_FOCUSED};

pub fn textarea_with_block<'a, 'b>(textarea: &'a mut TextArea<'b>, selected: bool, title: &str) -> &'a TextArea<'b> {
    textarea.set_block(get_block(selected, title));
    textarea
}

pub fn render_widget_with_textarea(
    title: &str,
    value: &str,
    is_selected: bool,
    textarea: Option<&mut TextArea>,
    frame: &mut Frame,
    rect: Rect,
) {
    if let (Some(textarea), true) = (textarea, is_selected) {
        frame.render_widget(textarea_with_block(textarea, is_selected, title).widget(), rect)
    } else {
        frame.render_widget(Paragraph::new(Text::raw(value)).block(get_block(is_selected, title)), rect)
    }
}

pub fn popup_title<'a>(popup: &PopupState) -> &'a str {
    match popup {
        PopupState::Confirm(_) => "Confirm",
        PopupState::Export(_) => "Export Contacts to vCard Format",
        PopupState::Import(_) => "Import Contacts from vCard Format",
        PopupState::Message(_) => "",
        PopupState::PropertyAdr(_) => "Edit Address",
        PopupState::PropertyBDay(_) => "Edit Birthday",
        PopupState::PropertyEmail(_) => "Edit Email Address",
        PopupState::PropertyExtra(_) => "Add a Custom Property",
        PopupState::PropertyN(_) => "Edit Name",
        PopupState::PropertyNote(_) => "Edit Note",
        PopupState::PropertyRemove(_) => "Remove Property",
        PopupState::PropertyTel(_) => "Edit Phone Number",
        PopupState::PropertyUrl(_) => "Edit Url",
        PopupState::VcardAdd(_) => "Add a New vCard",
    }
}

pub fn popup_size(popup: &mut PopupState) -> (u16, u16) {
    match popup {
        PopupState::Confirm(_) => (8, 40),
        PopupState::Export(_) => (20, 50),
        PopupState::Import(_) => (20, 50),
        PopupState::Message(_) => (8, 50),
        PopupState::PropertyAdr(_) => (14, 50),
        PopupState::PropertyBDay(_) => (8, 50),
        PopupState::PropertyEmail(_) => (8, 50),
        PopupState::PropertyExtra(_) => (14, 70),
        PopupState::PropertyN(_) => (11, 50),
        PopupState::PropertyNote(_) => (20, 50),
        PopupState::PropertyRemove(_) => (12, 50),
        PopupState::PropertyTel(_) => (8, 50),
        PopupState::PropertyUrl(_) => (8, 50),
        PopupState::VcardAdd(_) => (11, 50),
    }
}

pub fn property_title(name: &str) -> &str {
    match name {
        PropertyName::ADR => "Addresses",
        PropertyName::BDAY => "Birthday",
        PropertyName::EMAIL => "Emails",
        PropertyName::FN => "Full Name",
        PropertyName::NOTE => "Notes",
        PropertyName::N => "Name",
        PropertyName::TEL => "Phone Numbers",
        PropertyName::URL => "Websites",
        _ => name,
    }
}

pub fn property_content(property: &Property) -> String {
    match property.name() {
        PropertyName::ADR => {
            if let ValueListComponent(data) = &property.get_value() {
                format!(
                    "{}\n{} {}, {}\n{}",
                    data.value[2][0], data.value[3][0], data.value[4][0], data.value[5][0], data.value[6][0]
                )
            } else {
                property.get_value().to_string()
            }
        }
        PropertyName::N => {
            if let ValueListComponent(data) = &property.get_value().clone() {
                let values = [
                    &data.value[3][0],
                    &data.value[1][0],
                    &data.value[2][0],
                    &data.value[0][0],
                    &data.value[4][0],
                ];
                values
                    .into_iter()
                    .filter(|s| !s.is_empty())
                    .map(|a| a.to_owned())
                    .collect::<Vec<String>>()
                    .join(" ")
            } else {
                property.get_value().to_string()
            }
        }
        _ => property.get_value().to_string(),
    }
}

pub fn property_n_to_string(property_n: &Property) -> String {
    if let ValueListComponent(data) = property_n.get_value() {
        let string = [
            data.value.get(1).cloned().unwrap_or_default().join(","),
            data.value.get(2).cloned().unwrap_or_default().join(","),
            data.value.get(0).cloned().unwrap_or_default().join(","),
        ]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .join(" ");

        if !string.is_empty() {
            return string;
        }
    }

    String::from("No Name")
}

pub fn get_block<'a>(selected: bool, title: &str) -> Block<'a> {
    let borders_style = {
        if selected {
            Style::default().fg(PRIMARY_FOCUSED)
        } else {
            Style::default().fg(PRIMARY)
        }
    };

    Block::default()
        .title(title.to_string())
        .border_style(borders_style)
        .borders(Borders::ALL)
}

pub fn get_list_item<'a>(string: String, selected: bool) -> ListItem<'a> {
    let item = ListItem::new(string);

    let style = {
        if selected {
            Style::default().bg(PRIMARY).fg(PRIMARY_CONTRAST)
        } else {
            Style::default().fg(PRIMARY)
        }
    };

    item.style(style)
}
