use std::borrow::BorrowMut;

use ratatui::widgets::ListState;
use vcard_parser::constants::PropertyName;
use vcard_parser::traits::HasName;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::Vcard;

use crate::state::list::StatefulList;

#[derive(Clone, Default)]
pub struct ExtraPropertiesState {
    pub current: Option<Property>,
    pub list: ListState,
    pub properties: Vec<Property>,
}

impl StatefulList<Property> for ExtraPropertiesState {
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

impl From<&Vcard> for ExtraPropertiesState {
    fn from(vcard: &Vcard) -> Self {
        let mut properties = Vec::new();

        let restricted = [
            PropertyName::FN,
            PropertyName::N,
            PropertyName::BDAY,
            PropertyName::ADR,
            PropertyName::EMAIL,
            PropertyName::NOTE,
            PropertyName::TEL,
            PropertyName::URL,
            PropertyName::VERSION,
            PropertyName::UID,
        ];
        for property in vcard.get_properties() {
            if !restricted.contains(&property.name()) {
                properties.push(property.clone());
            }
        }

        Self {
            current: None,
            list: Default::default(),
            properties,
        }
    }
}
