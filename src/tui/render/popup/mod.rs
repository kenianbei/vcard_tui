use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Clear};
use ratatui::Frame;

use property_adr::render_property_adr_popup;
use property_email::render_property_email_popup;
use property_note::render_property_note_popup;
use property_remove::render_property_remove_popup;
use property_tel::render_property_tel_popup;
use property_url::render_property_url_popup;

use crate::state::popup::PopupState;
use crate::tui::render::popup::confirm::render_confirm_popup;
use crate::tui::render::popup::export::render_export_popup;
use crate::tui::render::popup::import::render_import_popup;
use crate::tui::render::popup::message::render_message_popup;
use crate::tui::render::popup::property_bday::render_property_bday_popup;
use crate::tui::render::popup::property_extra::render_property_extra_popup;
use crate::tui::render::popup::property_n::render_property_n_popup;
use crate::tui::render::popup::vcard_add::render_vcard_add_popup;
use crate::util::{popup_size, popup_title};
use crate::PRIMARY;

mod confirm;
mod export;
mod import;
mod message;
mod property_adr;
mod property_bday;
mod property_email;
mod property_extra;
mod property_n;
mod property_note;
mod property_remove;
mod property_tel;
mod property_url;
mod vcard_add;

pub fn render_popup(frame: &mut Frame, rect: Rect, popup: &mut PopupState) {
    let block = Block::default()
        .title(popup_title(popup))
        .borders(Borders::ALL)
        .style(Style::default().fg(PRIMARY));
    let (height, width) = popup_size(popup);
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((rect.height - height) / 2),
            Constraint::Length(height),
            Constraint::Length((rect.height - height) / 2),
        ])
        .split(rect);
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((rect.width - width) / 2),
            Constraint::Length(width),
            Constraint::Length((rect.width - width) / 2),
        ])
        .split(rows[1]);
    let center = columns[1];

    frame.render_widget(Clear, center);
    frame.render_widget(block, center);

    match popup {
        PopupState::Confirm(state) => render_confirm_popup(state, frame, center),
        PopupState::Export(state) => render_export_popup(state, frame, center),
        PopupState::Import(state) => render_import_popup(state, frame, center),
        PopupState::Message(state) => render_message_popup(state, frame, center),
        PopupState::PropertyAdr(state) => render_property_adr_popup(state, frame, center),
        PopupState::PropertyBDay(state) => render_property_bday_popup(state, frame, center),
        PopupState::PropertyEmail(state) => render_property_email_popup(state, frame, center),
        PopupState::PropertyExtra(state) => render_property_extra_popup(state, frame, center),
        PopupState::PropertyN(state) => render_property_n_popup(state, frame, center),
        PopupState::PropertyNote(state) => render_property_note_popup(state, frame, center),
        PopupState::PropertyRemove(state) => render_property_remove_popup(state, frame, center),
        PopupState::PropertyTel(state) => render_property_tel_popup(state, frame, center),
        PopupState::PropertyUrl(state) => render_property_url_popup(state, frame, center),
        PopupState::VcardAdd(state) => render_vcard_add_popup(state, frame, center),
    }
}
