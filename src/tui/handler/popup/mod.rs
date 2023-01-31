use crossterm::event::{Event, KeyCode};

use crate::state::popup::PopupState;
use crate::tui::handler::popup::confirm::handle_popup_confirm;
use crate::tui::handler::popup::export::handle_popup_export;
use crate::tui::handler::popup::import::handle_popup_import;
use crate::tui::handler::popup::message::handle_popup_message;
use crate::tui::handler::popup::property_adr::handle_popup_property_adr;
use crate::tui::handler::popup::property_bday::handle_popup_property_bday;
use crate::tui::handler::popup::property_email::handle_popup_property_email;
use crate::tui::handler::popup::property_extra::handle_popup_property_extra;
use crate::tui::handler::popup::property_n::handle_popup_property_n;
use crate::tui::handler::popup::property_note::handle_popup_property_note;
use crate::tui::handler::popup::property_remove::handle_popup_property_remove;
use crate::tui::handler::popup::property_tel::handle_popup_property_tel;
use crate::tui::handler::popup::property_url::handle_popup_property_url;
use crate::tui::handler::popup::vcard_add::handle_popup_vcard_add;
use crate::State;

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

pub fn handle_popup(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(popup) = &state.popup {
        match popup {
            PopupState::Confirm(_) => handle_popup_confirm(event, state)?,
            PopupState::Export(_) => handle_popup_export(event, state)?,
            PopupState::Import(_) => handle_popup_import(event, state)?,
            PopupState::Message(_) => handle_popup_message(event, state)?,
            PopupState::PropertyAdr(_) => handle_popup_property_adr(event, state)?,
            PopupState::PropertyBDay(_) => handle_popup_property_bday(event, state)?,
            PopupState::PropertyEmail(_) => handle_popup_property_email(event, state)?,
            PopupState::PropertyExtra(_) => handle_popup_property_extra(event, state)?,
            PopupState::PropertyN(_) => handle_popup_property_n(event, state)?,
            PopupState::PropertyNote(_) => handle_popup_property_note(event, state)?,
            PopupState::PropertyRemove(_) => handle_popup_property_remove(event, state)?,
            PopupState::PropertyTel(_) => handle_popup_property_tel(event, state)?,
            PopupState::PropertyUrl(_) => handle_popup_property_url(event, state)?,
            PopupState::VcardAdd(_) => handle_popup_vcard_add(event, state)?,
        };
        if let Event::Key(key_event) = event {
            if let KeyCode::Esc = key_event.code {
                state.popup = None
            }
        }
    }
    Ok(())
}
