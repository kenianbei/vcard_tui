use crossterm::event::{Event, KeyCode};

use crate::state::list::StatefulList;
use crate::state::popup::export::ExportSelected;
use crate::state::popup::PopupState;
use crate::state::selected::Select;
use crate::tui::HasTextArea;
use crate::State;

pub fn handle_popup_export(event: &Event, state: &mut State) -> anyhow::Result<()> {
    if let (Some(PopupState::Export(s)), Event::Key(key_event)) = (state.popup.as_mut(), event) {
        match key_event.code {
            KeyCode::Enter => match s.selected.get() {
                ExportSelected::Files => {
                    if let Some((path, _)) = &s.files.current {
                        if path.is_dir() {
                            s.files.path = path.clone();
                            s.files.refresh()?;
                        }
                    }
                }
                ExportSelected::TextArea => {
                    s.toggle_textarea();
                }
                ExportSelected::Export => {
                    if !s.value.is_empty() && s.files.path.is_dir() {
                        let path = s.files.path.join(&s.value);
                        state.contacts.export(path.to_str().unwrap())?;
                        state.popup = None;
                    }
                }
                ExportSelected::Cancel => state.popup = None,
            },
            KeyCode::Up => {
                if s.selected.is(ExportSelected::Files) {
                    s.files.prev()
                }
            }
            KeyCode::Down => {
                if s.selected.is(ExportSelected::Files) {
                    s.files.next()
                }
            }
            KeyCode::BackTab => s.selected.prev(),
            KeyCode::Tab => s.selected.next(),
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
    Ok(())
}
