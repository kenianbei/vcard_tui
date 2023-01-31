use crossterm::event::{Event, KeyCode};

use crate::state::popup::confirm::ConfirmState;
use crate::state::popup::export::ExportState;
use crate::state::popup::import::ImportState;
use crate::{PopupState, State};

pub mod contact;
pub mod contacts;
pub mod popup;

pub fn handle(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Event::Key(key_event) = event {
        match key_event.code {
            KeyCode::Char('e') => {
                if !state.contacts.vcards.is_empty() {
                    state.popup = Some(PopupState::Export(ExportState::default()))
                }
            }
            KeyCode::Char('i') => state.popup = Some(PopupState::Import(ImportState::default())),
            KeyCode::Char('q') => state.popup = Some(PopupState::Confirm(ConfirmState::default())),
            _ => {}
        }
    }
    Ok(())
}
