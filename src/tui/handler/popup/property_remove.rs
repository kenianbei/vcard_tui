use crossterm::event::{Event, KeyCode};

use crate::state::popup::property_remove::PopupRemoveSelected;
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::State;

pub fn handle_popup_property_remove(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(PopupState::PropertyRemove(s)) = state.popup.as_mut() {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => {
                    if s.selected.is(PopupRemoveSelected::Remove) {
                        state.contacts.remove_property(s.uuid)?;
                    }
                    state.popup = None;
                }
                KeyCode::Left | KeyCode::BackTab => s.selected.prev(),
                KeyCode::Right | KeyCode::Tab => s.selected.next(),
                _ => {}
            }
        }
    }
    Ok(())
}
