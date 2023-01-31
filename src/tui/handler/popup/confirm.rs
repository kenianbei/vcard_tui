use crossterm::event::{Event, KeyCode};

use crate::state::popup::confirm::{ConfirmKind, ConfirmSelected};
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::State;

pub fn handle_popup_confirm(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(PopupState::Confirm(s)) = state.popup.as_mut() {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => match s.selected.get() {
                    ConfirmSelected::Cancel => state.popup = None,
                    ConfirmSelected::Confirm => match s.kind {
                        ConfirmKind::Quit => state.render = false,
                        ConfirmKind::Delete => {
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
