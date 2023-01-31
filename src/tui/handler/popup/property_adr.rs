use crossterm::event::{Event, KeyCode};

use crate::state::list::StatefulList;
use crate::state::popup::property_adr::PropertyAdrSelected;
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::tui::HasTextArea;
use crate::State;

pub fn handle_popup_property_adr(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(PopupState::PropertyAdr(s)) = state.popup.as_mut() {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => {
                    if s.selected.is(PropertyAdrSelected::Save) {
                        let selected = state.contacts.current.properties_adr.list.selected();
                        s.property = state.contacts.update_property(&s.property)?;
                        state.popup = None;
                        state.contacts.current.properties_adr.set_selected(selected);
                    } else {
                        s.toggle_textarea();
                    }
                }
                KeyCode::BackTab => {
                    s.textarea = None;
                    s.selected.prev();
                }
                KeyCode::Tab => {
                    s.textarea = None;
                    s.selected.next();
                }
                KeyCode::Left => {
                    if let Some(textarea) = s.textarea.as_mut() {
                        textarea.input(*key_event);
                    } else {
                        s.selected.prev()
                    }
                }
                KeyCode::Right => {
                    if let Some(textarea) = s.textarea.as_mut() {
                        textarea.input(*key_event);
                    } else {
                        s.selected.next()
                    }
                }
                _ => {
                    if let Some(textarea) = s.textarea.as_mut() {
                        textarea.input(*key_event);
                    }
                }
            }
        }
    }
    Ok(())
}
