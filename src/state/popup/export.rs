use tui_textarea::TextArea;

use crate::state::files::{FilesExclude, FilesState};
use crate::state::selected::{Select, Selected};
use crate::tui::HasTextArea;

#[derive(Clone)]
pub struct ExportState<'a> {
    pub files: FilesState,
    pub selected: Selected<ExportSelected>,
    pub textarea: Option<TextArea<'a>>,
    pub value: String,
}

impl<'a> Default for ExportState<'a> {
    fn default() -> Self {
        Self {
            files: FilesState::new(FilesExclude::Files),
            selected: Selected::from(ExportSelected::TextArea),
            textarea: Some(TextArea::default()),
            value: String::new(),
        }
    }
}

impl<'a> HasTextArea<'a> for ExportState<'a> {
    fn textarea_value_get(&self) -> String {
        self.value.to_string()
    }
    fn textarea_value_set(&mut self, string: String) {
        if self.selected.is(ExportSelected::TextArea) {
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
pub enum ExportSelected {
    Cancel,
    Export,
    Files,
    TextArea,
}

impl Select for Selected<ExportSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            ExportSelected::TextArea => ExportSelected::TextArea,
            ExportSelected::Files => ExportSelected::TextArea,
            ExportSelected::Cancel => ExportSelected::Files,
            ExportSelected::Export => ExportSelected::Cancel,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            ExportSelected::TextArea => ExportSelected::Files,
            ExportSelected::Files => ExportSelected::Cancel,
            ExportSelected::Cancel => ExportSelected::Export,
            ExportSelected::Export => ExportSelected::Export,
        })
    }
}
