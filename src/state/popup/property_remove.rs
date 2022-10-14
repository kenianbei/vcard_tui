use vcard_parser::uuid::Uuid;
use vcard_parser::vcard::property::Property;

use crate::state::selected::{Select, Selected};

#[derive(Clone)]
pub struct PropertyRemoveState {
    pub selected: Selected<PopupRemoveSelected>,
    pub uuid: Uuid,
}

impl From<&Property> for PropertyRemoveState {
    fn from(property: &Property) -> Self {
        Self {
            selected: Selected::from(PopupRemoveSelected::Remove),
            uuid: property.get_uuid(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum PopupRemoveSelected {
    Cancel,
    Remove,
}

impl Select for Selected<PopupRemoveSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PopupRemoveSelected::Cancel => PopupRemoveSelected::Cancel,
            PopupRemoveSelected::Remove => PopupRemoveSelected::Cancel,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PopupRemoveSelected::Cancel => PopupRemoveSelected::Remove,
            PopupRemoveSelected::Remove => PopupRemoveSelected::Remove,
        })
    }
}
