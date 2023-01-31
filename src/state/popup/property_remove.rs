use vcard_parser::vcard::property::Property;

use crate::state::selected::{Select, Selected};

#[derive(Clone)]
pub struct PropertyRemoveState {
    pub selected: Selected<PropertyRemoveSelected>,
    pub property: Property,
}

impl From<&Property> for PropertyRemoveState {
    fn from(property: &Property) -> Self {
        Self {
            selected: Selected::from(PropertyRemoveSelected::Remove),
            property: property.clone(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum PropertyRemoveSelected {
    Cancel,
    Remove,
}

impl Select for Selected<PropertyRemoveSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PropertyRemoveSelected::Cancel => PropertyRemoveSelected::Cancel,
            PropertyRemoveSelected::Remove => PropertyRemoveSelected::Cancel,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PropertyRemoveSelected::Cancel => PropertyRemoveSelected::Remove,
            PropertyRemoveSelected::Remove => PropertyRemoveSelected::Remove,
        })
    }
}
