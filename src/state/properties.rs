use std::borrow::BorrowMut;

use ratatui::widgets::ListState;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::Vcard;

use crate::state::list::StatefulList;

#[derive(Clone)]
pub struct PropertiesState {
    pub current: Option<Property>,
    pub list: ListState,
    pub property_name: String,
    pub properties: Vec<Property>,
}

impl StatefulList<Property> for PropertiesState {
    fn list(&mut self) -> &mut ListState {
        self.list.borrow_mut()
    }
    fn get(&self) -> Option<&Property> {
        if let Some(index) = self.list.selected() {
            self.properties.get(index)
        } else {
            None
        }
    }
    fn set(&mut self, current: &Property) {
        self.current = Some(current.clone());
    }
    fn vec(&self) -> Vec<Property> {
        self.properties.clone()
    }
}

impl From<(&str, &Vcard)> for PropertiesState {
    fn from((property_name, vcard): (&str, &Vcard)) -> Self {
        Self {
            current: None,
            list: Default::default(),
            properties: vcard.get_properties_by_name(property_name),
            property_name: property_name.to_string(),
        }
    }
}

impl From<&str> for PropertiesState {
    fn from(property_name: &str) -> Self {
        Self {
            current: None,
            list: Default::default(),
            property_name: property_name.to_string(),
            properties: Vec::new(),
        }
    }
}
