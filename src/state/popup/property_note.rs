use tui_textarea::TextArea;
use vcard_parser::traits::HasValue;
use vcard_parser::vcard::property::property_note::PropertyNoteData;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::property::Property::PropertyNote;

use crate::state::selected::{Select, Selected};

#[derive(Clone)]
pub struct PropertyNoteState<'a> {
    pub property: Property,
    pub selected: Selected<PropertyNoteSelected>,
    pub textarea: TextArea<'a>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum PropertyNoteSelected {
    Note,
    Save,
}

impl<'a> Default for PropertyNoteState<'a> {
    fn default() -> Self {
        Self {
            property: PropertyNote(PropertyNoteData::default()),
            selected: Selected::from(PropertyNoteSelected::Note),
            textarea: TextArea::default(),
        }
    }
}

impl<'a> From<&Property> for PropertyNoteState<'a> {
    fn from(property: &Property) -> Self {
        Self {
            property: property.clone(),
            selected: Selected::from(PropertyNoteSelected::Note),
            textarea: TextArea::new(
                property
                    .get_value()
                    .to_string()
                    .split(r"\\n")
                    .map(|s| s.to_string())
                    .into_iter()
                    .collect::<Vec<String>>(),
            ),
        }
    }
}

impl Select for Selected<PropertyNoteSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PropertyNoteSelected::Note => PropertyNoteSelected::Note,
            PropertyNoteSelected::Save => PropertyNoteSelected::Note,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PropertyNoteSelected::Note => PropertyNoteSelected::Save,
            PropertyNoteSelected::Save => PropertyNoteSelected::Save,
        })
    }
}
