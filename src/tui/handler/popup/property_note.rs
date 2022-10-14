use crossterm::event::{Event, KeyCode};

use crate::state::popup::property_note::PopupNoteSelected;
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::State;

pub fn handle_popup_property_note(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(PopupState::PropertyNote(s)) = state.popup.as_mut() {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => match s.selected.get() {
                    PopupNoteSelected::Note => {
                        s.textarea.input(*key_event);
                    }
                    PopupNoteSelected::Save => {
                        s.values.note = s.textarea.lines().join("\\n");
                        state.contacts.update_property(s.uuid, s.to_property_string().as_str())?;
                        state.popup = None
                    }
                },
                KeyCode::BackTab => {
                    s.selected.prev();
                }
                KeyCode::Tab => {
                    s.selected.next();
                }
                _ => {
                    s.textarea.input(*key_event);
                }
            }
        }
    }
    Ok(())
}
