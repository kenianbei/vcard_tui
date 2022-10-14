use crate::state::files::{FilesExclude, FilesState};
use crate::state::selected::{Select, Selected};

#[derive(Clone)]
pub struct ImportState {
    pub files: FilesState,
    pub selected: Selected<PopupImportSelected>,
}

impl Default for ImportState {
    fn default() -> Self {
        Self {
            files: FilesState::new(FilesExclude::None),
            selected: Selected::from(PopupImportSelected::Files),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum PopupImportSelected {
    Files,
    Cancel,
    Import,
}

impl Select for Selected<PopupImportSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PopupImportSelected::Files => PopupImportSelected::Files,
            PopupImportSelected::Cancel => PopupImportSelected::Files,
            PopupImportSelected::Import => PopupImportSelected::Cancel,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PopupImportSelected::Files => PopupImportSelected::Cancel,
            PopupImportSelected::Cancel => PopupImportSelected::Import,
            PopupImportSelected::Import => PopupImportSelected::Import,
        })
    }
}
