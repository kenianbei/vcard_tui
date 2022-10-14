use tui_textarea::TextArea;

use crate::state::files::{FilesExclude, FilesState};
use crate::state::selected::{Select, Selected};
use crate::tui::HasTextArea;

#[derive(Clone)]
pub struct ExportState<'a> {
    pub files: FilesState,
    pub selected: Selected<PopupExportSelected>,
    pub textarea: Option<TextArea<'a>>,
    pub value: String,
}

impl<'a> Default for ExportState<'a> {
    fn default() -> Self {
        Self {
            files: FilesState::new(FilesExclude::Files),
            selected: Selected::from(PopupExportSelected::TextArea),
            textarea: Some(TextArea::default()),
            value: String::new(),
        }
    }
}

impl<'a> HasTextArea<'a> for ExportState<'a> {
    fn value_get(&self) -> String {
        self.value.to_string()
    }
    fn value_set(&mut self, string: String) {
        if self.selected.is(PopupExportSelected::TextArea) {
            self.value = string;
        }
    }
    fn textarea_get(&self) -> &Option<TextArea> {
        &self.textarea
    }
    fn textarea_set(&mut self, textarea: Option<TextArea<'a>>) {
        self.textarea = textarea
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum PopupExportSelected {
    Cancel,
    Export,
    Files,
    TextArea,
}

impl Select for Selected<PopupExportSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PopupExportSelected::TextArea => PopupExportSelected::TextArea,
            PopupExportSelected::Files => PopupExportSelected::TextArea,
            PopupExportSelected::Cancel => PopupExportSelected::Files,
            PopupExportSelected::Export => PopupExportSelected::Cancel,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PopupExportSelected::TextArea => PopupExportSelected::Files,
            PopupExportSelected::Files => PopupExportSelected::Cancel,
            PopupExportSelected::Cancel => PopupExportSelected::Export,
            PopupExportSelected::Export => PopupExportSelected::Export,
        })
    }
}
