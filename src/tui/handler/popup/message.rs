use crossterm::event::{Event, KeyCode};

use crate::state::popup::PopupState;
use crate::State;

pub fn handle_popup_message(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(PopupState::Message(_)) = state.popup.as_mut() {
        if let Event::Key(key_event) = event {
            if key_event.code == KeyCode::Enter {
                state.popup = None;
            }
        }
    }
    Ok(())
}
