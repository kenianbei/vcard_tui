use tui_textarea::TextArea;
use vcard_parser::uuid::Uuid;

use crate::state::selected::{Select, Selected};
use crate::tui::HasTextArea;

#[derive(Clone)]
pub struct PropertyExtraState<'a> {
    pub selected: Selected<PopupExtraSelected>,
    pub textarea: Option<TextArea<'a>>,
    pub uuid: Option<Uuid>,
    pub value: String,
}

impl<'a> Default for PropertyExtraState<'a> {
    fn default() -> Self {
        Self {
            selected: Selected::from(PopupExtraSelected::Extra),
            textarea: Some(TextArea::default()),
            uuid: None,
            value: String::new(),
        }
    }
}

impl<'a> HasTextArea<'a> for PropertyExtraState<'a> {
    fn value_get(&self) -> String {
        match self.selected.get() {
            PopupExtraSelected::Extra => self.value.to_string(),
            PopupExtraSelected::Save => String::new(),
        }
    }
    fn value_set(&mut self, string: String) {
        match self.selected.get() {
            PopupExtraSelected::Extra => self.value = string,
            PopupExtraSelected::Save => {}
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
pub enum PopupExtraSelected {
    Extra,
    Save,
}

impl Select for Selected<PopupExtraSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PopupExtraSelected::Extra => PopupExtraSelected::Extra,
            PopupExtraSelected::Save => PopupExtraSelected::Extra,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PopupExtraSelected::Extra => PopupExtraSelected::Save,
            PopupExtraSelected::Save => PopupExtraSelected::Save,
        })
    }
}
