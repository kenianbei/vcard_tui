use std::borrow::BorrowMut;

use tui::widgets::ListState;
use vcard_parser::vcard::property::types::PropertyType;
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

        let restricted = [PropertyType::Fn, PropertyType::N, PropertyType::BDay, PropertyType::Adr, PropertyType::Email, PropertyType::Note, PropertyType::Tel, PropertyType::Url, PropertyType::Version, PropertyType::Uid];
        for property in vcard.get_properties() {
            if !restricted.contains(property.get_type()) {
                properties.push(property.clone())
            }
        }

        Self {
            current: None,
            list: Default::default(),
            properties,
        }
    }
}
