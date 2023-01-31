use crossterm::event::{Event, KeyCode};
use vcard_parser::traits::HasValue;
use vcard_parser::vcard::value::value_text::ValueTextData;
use vcard_parser::vcard::value::Value::ValueText;

use crate::state::list::StatefulList;
use crate::state::popup::property_note::PropertyNoteSelected;
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::State;

pub fn handle_popup_property_note(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(PopupState::PropertyNote(s)) = state.popup.as_mut() {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => match s.selected.get() {
                    PropertyNoteSelected::Note => {
                        s.textarea.input(*key_event);
                    }
                    PropertyNoteSelected::Save => {
                        s.property
                            .set_value(ValueText(ValueTextData {
                                value: s.textarea.lines().join("\\n"),
                            }))
                            .ok();

                        let selected = state.contacts.current.properties_note.list.selected();
                        s.property = state.contacts.update_property(&s.property)?;
                        state.popup = None;
                        state.contacts.current.properties_note.set_selected(selected);
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
