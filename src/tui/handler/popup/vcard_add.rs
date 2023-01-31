use crossterm::event::{Event, KeyCode};
use vcard_parser::vcard::Vcard;

use crate::state::popup::property_n::PropertyNSelected;
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::tui::{HasTextArea, HasValueListComponent};
use crate::{MessageState, State};

pub fn handle_popup_vcard_add(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(PopupState::VcardAdd(s)) = state.popup.as_mut() {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => {
                    if s.property_n.selected.is(PropertyNSelected::Save) {
                        let mut vcard = Vcard::new(
                            format!(
                                "{} {} {}",
                                s.property_n.value_listcomponent_get_string(1),
                                s.property_n.value_listcomponent_get_string(2),
                                s.property_n.value_listcomponent_get_string(0)
                            )
                            .as_str(),
                        );
                        match vcard.set_property(&s.property_n.property) {
                            Ok(_) => {
                                state.contacts.add(vcard)?;
                                state.popup = None;
                            }
                            Err(err) => {
                                state.popup = Some(PopupState::Message(MessageState::from(err.to_string())));
                            }
                        }
                    } else {
                        s.property_n.toggle_textarea();
                    }
                }
                KeyCode::BackTab => s.property_n.selected.prev(),
                KeyCode::Left => s.property_n.selected.prev(),
                KeyCode::Right => s.property_n.selected.next(),
                KeyCode::Tab => s.property_n.selected.next(),
                _ => {
                    if let Some(textarea) = s.property_n.textarea.as_mut() {
                        textarea.input(*key_event);
                    }
                }
            }
        }
    }
    Ok(())
}
