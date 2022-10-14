use crossterm::event::{Event, KeyCode};

use crate::state::popup::confirm::{PopupConfirmKind, PopupConfirmSelected};
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::State;

pub fn handle_popup_confirm(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(PopupState::Confirm(s)) = state.popup.as_mut() {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => match s.selected.get() {
                    PopupConfirmSelected::Cancel => state.popup = None,
                    PopupConfirmSelected::Confirm => match s.kind {
                        PopupConfirmKind::Quit => state.render = false,
                        PopupConfirmKind::Delete => {
                            state.contacts.delete()?;
                            state.popup = None;
                        }
                    },
                },
                KeyCode::Left | KeyCode::BackTab => s.selected.prev(),
                KeyCode::Right | KeyCode::Tab => s.selected.next(),
                _ => {}
            }
        }
    }
    Ok(())
}
