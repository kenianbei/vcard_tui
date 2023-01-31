use crossterm::event::{Event, KeyCode};
use vcard_parser::vcard::property::property_fn::PropertyFnData;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::property::Property::PropertyFn;

use crate::state::popup::property_n::PropertyNSelected;
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::tui::HasTextArea;
use crate::util::property_n_to_string;
use crate::State;

pub fn handle_popup_property_n(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(PopupState::PropertyN(s)) = state.popup.as_mut() {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => {
                    if s.selected.is(PropertyNSelected::Save) {
                        s.property = state.contacts.update_property(&s.property)?;
                        let property_fn = Property::try_from(format!("FN:{}\n", property_n_to_string(&s.property)).as_str())
                            .unwrap_or(PropertyFn(PropertyFnData::from("No Name")));
                        state.contacts.update_property(&property_fn)?;
                        state.popup = None
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
