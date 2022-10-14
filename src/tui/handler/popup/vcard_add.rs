use crossterm::event::{Event, KeyCode};
use vcard_parser::vcard::Vcard;

use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::State;

pub fn handle_popup_vcard_add(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(PopupState::VcardAdd(s)) = state.popup.as_mut() {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => {
                    let fullname = s.textarea.lines().first().unwrap().as_str();
                    state.contacts.add(Vcard::from_fullname(fullname)?)?;
                    state.popup = None;
                }
                KeyCode::BackTab => s.selected.prev(),
                KeyCode::Left => s.selected.prev(),
                KeyCode::Right => s.selected.next(),
                KeyCode::Tab => s.selected.next(),
                _ => {
                    s.textarea.input(*key_event);
                }
            }
        }
    }
    Ok(())
}
