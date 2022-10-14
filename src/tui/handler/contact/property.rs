use std::borrow::BorrowMut;

use crossterm::event::{Event, KeyCode};

use crate::state::contact::ContactSelected;
use crate::state::selected::Select;
use crate::tui::HasTextArea;
use crate::State;

pub fn handle_contact_property(event: &Event, state: &mut State) -> anyhow::Result<()> {
    let property = match state.contacts.current.selected.get() {
        ContactSelected::BDay => Some(state.contacts.current.property_bday.borrow_mut()),
        ContactSelected::Fn => Some(state.contacts.current.property_fn.borrow_mut()),
        _ => None,
    };

    if let (Some(s), Event::Key(key_event)) = (property, event) {
        match key_event.code {
            KeyCode::Left | KeyCode::BackTab => {
                if let Some(textarea) = s.textarea.as_mut() {
                    textarea.input(*key_event);
                } else if state.contacts.current.selected.is(ContactSelected::Fn) {
                    state.selected.prev()
                } else {
                    state.contacts.current.selected.prev();
                    state.contacts.current.set_selected_properties_to_first();
                }
            }
            KeyCode::Right | KeyCode::Tab => {
                if let Some(textarea) = s.textarea.as_mut() {
                    textarea.input(*key_event);
                } else {
                    state.contacts.current.selected.next();
                    state.contacts.current.set_selected_properties_to_first();
                }
            }
            KeyCode::Up => {
                if s.textarea.is_none() {
                    state.contacts.current.up();
                }
            }
            KeyCode::Down => {
                if s.textarea.is_none() {
                    state.contacts.current.down();
                }
            }
            KeyCode::Enter => {
                s.toggle_textarea();
                if s.textarea.is_none() {
                    let uuid = s.uuid;
                    let string = s.to_property_string();
                    state.contacts.update_property(uuid, string.as_str())?;
                }
            }
            _ => {
                if let Some(textarea) = s.textarea.as_mut() {
                    textarea.input(Event::Key(*key_event));
                }
            }
        }
    }

    Ok(())
}
