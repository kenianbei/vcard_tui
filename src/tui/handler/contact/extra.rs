use std::borrow::BorrowMut;

use crossterm::event::{Event, KeyCode};

use crate::state::list::StatefulList;
use crate::state::popup::property_extra::PropertyExtraState;
use crate::state::popup::property_remove::PropertyRemoveState;
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::State;

pub fn handle_contact_extra_properties(event: &Event, state: &mut State) {
    let s = state.contacts.current.properties_extra.borrow_mut();
    if let Event::Key(key_event) = event {
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
            KeyCode::Char('a') => state.popup = Some(PopupState::PropertyExtra(PropertyExtraState::default())),
            KeyCode::Char('d') => {
                if let Some(property) = &s.current {
                    state.popup = Some(PopupState::PropertyRemove(PropertyRemoveState::from(property)));
                }
            }
            _ => {}
        }
    }
}
