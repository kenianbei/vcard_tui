use tui_textarea::TextArea;
use vcard_parser::uuid::Uuid;
use vcard_parser::vcard::property::Property;

use crate::state::selected::{Select, Selected};

#[derive(Clone)]
pub struct PropertyNoteState<'a> {
    pub selected: Selected<PopupNoteSelected>,
    pub textarea: TextArea<'a>,
    pub values: PopupNoteValues,
    pub uuid: Option<Uuid>,
}

impl<'a> PropertyNoteState<'a> {
    pub fn to_property_string(&self) -> String {
        format!("NOTE:{}", self.values.note)
    }
}

impl<'a> Default for PropertyNoteState<'a> {
    fn default() -> Self {
        Self {
            selected: Selected::from(PopupNoteSelected::Note),
            textarea: TextArea::default(),
            uuid: None,
            values: PopupNoteValues::default(),
        }
    }
}

impl<'a> From<&Property> for PropertyNoteState<'a> {
    fn from(property: &Property) -> Self {
        Self {
            selected: Selected::from(PopupNoteSelected::Note),
            textarea: TextArea::new(Vec::from([property.get_value().to_string()])),
            uuid: Some(property.get_uuid()),
            values: PopupNoteValues {
                note: property.get_value().to_string(),
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum PopupNoteSelected {
    Note,
    Save,
}

impl Select for Selected<PopupNoteSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PopupNoteSelected::Note => PopupNoteSelected::Note,
            PopupNoteSelected::Save => PopupNoteSelected::Note,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PopupNoteSelected::Note => PopupNoteSelected::Save,
            PopupNoteSelected::Save => PopupNoteSelected::Save,
        })
    }
}

#[derive(Clone, Default)]
pub struct PopupNoteValues {
    pub note: String,
}
