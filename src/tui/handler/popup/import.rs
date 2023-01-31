use crossterm::event::{Event, KeyCode};

use crate::state::list::StatefulList;
use crate::state::popup::import::ImportSelected;
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::State;

pub fn handle_popup_import(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let Some(PopupState::Import(s)) = state.popup.as_mut() {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => match s.selected.get() {
                    ImportSelected::Files | ImportSelected::Import => {
                        if let Some((path, _)) = &s.files.current {
                            if path.is_dir() {
                                s.files.path = path.clone();
                                s.files.refresh()?;
                            } else {
                                state.contacts.import(path.to_str().unwrap())?;
                                state.popup = None;
                            }
                        }
                    }
                    _ => state.popup = None,
                },
                KeyCode::Up => s.files.prev(),
                KeyCode::Down => s.files.next(),
                KeyCode::BackTab => s.selected.prev(),
                KeyCode::Left => s.selected.prev(),
                KeyCode::Right => s.selected.next(),
                KeyCode::Tab => s.selected.next(),
                _ => {}
            }
        }
    }
    Ok(())
}
