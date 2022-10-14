use crossterm::event::{Event, KeyCode};

use crate::state::list::StatefulList;
use crate::state::popup::property_remove::PropertyRemoveState;
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::State;

pub fn handle_contact_properties(event: &Event, state: &mut State) {
    if let (Some(s), Event::Key(key_event)) = (state.contacts.current.get_selected_properties_state(), event) {
        match key_event.code {
            KeyCode::Left | KeyCode::BackTab => {
                state.contacts.current.selected.prev();
                state.contacts.current.set_selected_properties_to_first();
            }
            KeyCode::Right | KeyCode::Tab => {
                state.contacts.current.selected.next();
                state.contacts.current.set_selected_properties_to_first();
            }
            KeyCode::Up => {
                if s.properties.len() <= 1 {
                    state.contacts.current.up()
                } else {
                    s.prev()
                }
            }
            KeyCode::Down => {
                if s.properties.len() <= 1 {
                    state.contacts.current.down();
                } else {
                    s.next()
                }
            }
            KeyCode::Enter => {
                if let Some(property) = &s.current {
                    state.popup = Some(PopupState::from(property));
                }
            }
            KeyCode::Char(char) => match char {
                'a' => state.popup = Some(PopupState::from(&s.property_type)),
                'd' => {
                    if let Some(property) = &s.current {
                        state.popup = Some(PopupState::PropertyRemove(PropertyRemoveState::from(property)));
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
