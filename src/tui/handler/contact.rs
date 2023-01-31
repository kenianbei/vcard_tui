use crossterm::event::{Event, KeyCode};
use vcard_parser::constants::PropertyName;
use vcard_parser::traits::HasName;

use crate::state::contact::ContactSelected;
use crate::state::popup::property_bday::PropertyBDayState;
use crate::state::popup::property_extra::PropertyExtraState;
use crate::state::popup::property_n::PropertyNState;
use crate::state::popup::property_remove::PropertyRemoveState;
use crate::state::selected::Select;
use crate::{PopupState, State};

pub fn handle_contact(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Event::Key(key_event) = event {
        match key_event.code {
            KeyCode::Left | KeyCode::BackTab => {
                if state.contacts.current.selected.is(ContactSelected::N) {
                    state.selected.prev()
                } else {
                    state.contacts.current.selected.prev();
                    state.contacts.current.set_selected_properties_to_first();
                }
            }
            KeyCode::Right | KeyCode::Tab => {
                state.contacts.current.selected.next();
                state.contacts.current.set_selected_properties_to_first();
            }
            KeyCode::Up => {
                state.contacts.current.up();
            }
            KeyCode::Down => {
                state.contacts.current.down();
            }
            KeyCode::Enter => match state.contacts.current.selected.get() {
                ContactSelected::N => {
                    if let Some(p) = state.contacts.current.get_selected_property() {
                        state.popup = Some(PopupState::from(p));
                    } else {
                        state.popup = Some(PopupState::PropertyN(PropertyNState::default()))
                    }
                }
                ContactSelected::BDay => {
                    if let Some(p) = state.contacts.current.get_selected_property() {
                        state.popup = Some(PopupState::from(p));
                    } else {
                        state.popup = Some(PopupState::PropertyBDay(PropertyBDayState::default()))
                    }
                }
                ContactSelected::Extra => {
                    if let Some(p) = state.contacts.current.get_selected_property() {
                        state.popup = Some(PopupState::from(p));
                    }
                }
                _ => {
                    if let Some(p) = state.contacts.current.get_selected_property() {
                        state.popup = Some(PopupState::from(p));
                    }
                }
            },
            KeyCode::Char(char) => match char {
                'a' => {
                    if let Some(p) = state.contacts.current.get_selected_property() {
                        state.popup = Some(PopupState::from(p.name()));
                    } else {
                        state.popup = match state.contacts.current.selected.get() {
                            ContactSelected::N => Some(PopupState::from(PropertyName::N)),
                            ContactSelected::BDay => Some(PopupState::from(PropertyName::BDAY)),
                            ContactSelected::Email => Some(PopupState::from(PropertyName::EMAIL)),
                            ContactSelected::Tel => Some(PopupState::from(PropertyName::TEL)),
                            ContactSelected::Adr => Some(PopupState::from(PropertyName::ADR)),
                            ContactSelected::Url => Some(PopupState::from(PropertyName::URL)),
                            ContactSelected::Note => Some(PopupState::from(PropertyName::NOTE)),
                            ContactSelected::Extra => Some(PopupState::PropertyExtra(PropertyExtraState::default())),
                        };
                    }
                }
                'd' => {
                    if let Some(p) = state.contacts.current.get_selected_property() {
                        state.popup = Some(PopupState::PropertyRemove(PropertyRemoveState::from(p)));
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}
