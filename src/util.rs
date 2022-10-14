use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::style::Style;
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;
use tui_textarea::TextArea;
use vcard_parser::vcard::property::types::PropertyType;

use crate::state::popup::PopupState;
use crate::{PRIMARY, PRIMARY_FOCUSED};

pub fn textarea_with_block<'a, 'b>(textarea: &'a mut TextArea<'b>, selected: bool, title: &str) -> &'a TextArea<'b> {
    textarea.set_block(get_block(selected, title));
    textarea
}

pub fn render_widget_with_textarea(title: &str, value: &str, is_selected: bool, textarea: Option<&mut TextArea>, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
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
        PopupState::PropertyEmail(_) => "Edit Email Address",
        PopupState::PropertyExtra(_) => "Add a Custom Property",
        PopupState::PropertyNote(_) => "Edit Note",
        PopupState::PropertyRemove(_) => "Remove Property",
        PopupState::PropertyTel(_) => "Edit Phone Number",
        PopupState::PropertyUrl(_) => "Edit Url",
        PopupState::VcardAdd(_) => "Add a New vCard",
    }
}

pub fn popup_size(popup: &mut PopupState) -> (u16, u16) {
    match popup {
        PopupState::Confirm(_) => (8, 70),
        PopupState::Export(_) => (20, 50),
        PopupState::Import(_) => (20, 50),
        PopupState::Message(_) => (8, 50),
        PopupState::PropertyAdr(_) => (14, 50),
        PopupState::PropertyEmail(_) => (8, 50),
        PopupState::PropertyExtra(_) => (14, 70),
        PopupState::PropertyNote(_) => (20, 50),
        PopupState::PropertyRemove(_) => (12, 50),
        PopupState::PropertyTel(_) => (8, 50),
        PopupState::PropertyUrl(_) => (8, 50),
        PopupState::VcardAdd(_) => (8, 50),
    }
}

pub fn property_title<'a>(property_type: &PropertyType) -> &'a str {
    match property_type {
        PropertyType::Adr => "Addresses",
        PropertyType::Anniversary => "",
        PropertyType::BDay => "Birthday",
        PropertyType::BirthPlace => "",
        PropertyType::CalAdrUri => "",
        PropertyType::CalUri => "",
        PropertyType::Categories => "",
        PropertyType::ClientPidMap => "",
        PropertyType::ContactUri => "",
        PropertyType::DeathDate => "",
        PropertyType::DeathPlace => "",
        PropertyType::Email => "Emails",
        PropertyType::Expertise => "",
        PropertyType::FbUrl => "",
        PropertyType::Fn => "Full Name",
        PropertyType::Gender => "",
        PropertyType::Geo => "",
        PropertyType::Hobby => "",
        PropertyType::Impp => "",
        PropertyType::Interest => "",
        PropertyType::Key => "",
        PropertyType::Kind => "",
        PropertyType::Lang => "",
        PropertyType::Logo => "",
        PropertyType::Member => "",
        PropertyType::NickName => "",
        PropertyType::Note => "Notes",
        PropertyType::N => "",
        PropertyType::OrgDirectory => "",
        PropertyType::Org => "",
        PropertyType::Photo => "",
        PropertyType::ProdId => "",
        PropertyType::Related => "",
        PropertyType::Rev => "",
        PropertyType::Role => "",
        PropertyType::Sound => "",
        PropertyType::Source => "",
        PropertyType::Tel => "Phone Numbers",
        PropertyType::Title => "",
        PropertyType::Tz => "",
        PropertyType::Uid => "",
        PropertyType::Url => "Websites",
        PropertyType::Version => "",
        PropertyType::Xml => "",
    }
}

pub fn get_block<'a>(selected: bool, title: &str) -> Block<'a> {
    let borders_style = {
        if selected {
            Style::default().fg(PRIMARY_FOCUSED)
        } else {
            Style::default().fg(PRIMARY)
        }
    };

    Block::default().title(title.to_string()).border_style(borders_style).borders(Borders::ALL)
}
