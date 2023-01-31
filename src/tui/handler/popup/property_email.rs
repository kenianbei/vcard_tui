use crossterm::event::{Event, KeyCode};

use crate::state::list::StatefulList;
use crate::state::popup::property_email::PropertyEmailSelected;
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::tui::HasTextArea;
use crate::State;

pub fn handle_popup_property_email(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(PopupState::PropertyEmail(s)) = state.popup.as_mut() {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => {
                    if s.selected.is(PropertyEmailSelected::Save) {
                        let selected = state.contacts.current.properties_email.list.selected();
                        s.property = state.contacts.update_property(&s.property)?;
                        state.popup = None;
                        state.contacts.current.properties_email.set_selected(selected);
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
