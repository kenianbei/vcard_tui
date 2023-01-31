use crossterm::event::{Event, KeyCode};
use vcard_parser::traits::HasParameters;
use vcard_parser::vcard::property::Property;

use crate::state::list::StatefulList;
use crate::state::popup::property_extra::PropertyExtraSelected;
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::tui::HasTextArea;
use crate::{MessageState, State};

pub fn handle_popup_property_extra(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(PopupState::PropertyExtra(s)) = state.popup.as_mut() {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => {
                    if s.selected.is(PropertyExtraSelected::Save) {
                        let selected = state.contacts.current.properties_extra.list.selected();

                        match Property::try_from(format!("{}{}:{}\n", s.name, s.parameters, s.value).as_str()) {
                            Ok(mut property) => {
                                if let Some(parameter) = &s.pid {
                                    property.add_parameter(parameter.clone()).ok();
                                }
                                state.contacts.update_property(&property)?;
                                state.popup = None;
                            }
                            Err(err) => {
                                state.popup = Some(PopupState::Message(MessageState::from(err.to_string())));
                            }
                        }

                        state.contacts.current.properties_extra.set_selected(selected);
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
