use crate::state::files::{FilesExclude, FilesState};
use crate::state::selected::{Select, Selected};

#[derive(Clone)]
pub struct ImportState {
    pub files: FilesState,
    pub selected: Selected<ImportSelected>,
}

impl Default for ImportState {
    fn default() -> Self {
        Self {
            files: FilesState::new(FilesExclude::None),
            selected: Selected::from(ImportSelected::Files),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum ImportSelected {
    Files,
    Cancel,
    Import,
}

impl Select for Selected<ImportSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            ImportSelected::Files => ImportSelected::Files,
            ImportSelected::Cancel => ImportSelected::Files,
            ImportSelected::Import => ImportSelected::Cancel,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            ImportSelected::Files => ImportSelected::Cancel,
            ImportSelected::Cancel => ImportSelected::Import,
            ImportSelected::Import => ImportSelected::Import,
        })
    }
}
