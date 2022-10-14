use crossterm::event::{Event, KeyCode};

use crate::state::list::StatefulList;
use crate::state::popup::confirm::{ConfirmState, PopupConfirmKind, PopupConfirmSelected};
use crate::state::popup::vcard_add::VcardAddState;
use crate::state::popup::PopupState;
use crate::state::selected::{Select, Selected};
use crate::State;

pub fn handle_contacts(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Event::Key(key_event) = event {
        match key_event.code {
            KeyCode::Right | KeyCode::Tab => {
                if state.contacts.current.property_fn.uuid.is_some() {
                    state.selected.next();
                }
            }
            KeyCode::Up => state.contacts.prev(),
            KeyCode::Down => state.contacts.next(),
            KeyCode::Home => state.contacts.home(),
            KeyCode::End => state.contacts.end(),
            KeyCode::PageUp => state.contacts.pageup(),
            KeyCode::PageDown => state.contacts.pagedown(),
            KeyCode::Delete | KeyCode::Backspace | KeyCode::Char('d') => {
                if state.contacts.current.property_fn.uuid.is_some() {
                    state.popup = Some(PopupState::Confirm(ConfirmState {
                        kind: PopupConfirmKind::Delete,
                        selected: Selected::from(PopupConfirmSelected::Confirm),
                    }))
                }
            }
            KeyCode::Char('a') => state.popup = Some(PopupState::VcardAdd(VcardAddState::default())),
            KeyCode::Char('s') => state.popup = Some(PopupState::VcardAdd(VcardAddState::default())),
            _ => {}
        }
    }
    Ok(())
}
