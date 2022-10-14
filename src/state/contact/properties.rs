use std::borrow::BorrowMut;

use tui::widgets::ListState;
use vcard_parser::vcard::property::types::PropertyType;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::values::data::ValueData;
use vcard_parser::vcard::Vcard;

use crate::state::list::StatefulList;

#[derive(Clone)]
pub struct PropertiesState {
    pub current: Option<Property>,
    pub list: ListState,
    pub property_type: PropertyType,
    pub properties: Vec<Property>,
}

impl StatefulList<Property> for PropertiesState {
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

impl PropertiesState {
    pub fn get_content(&self, property: &Property) -> String {
        match self.property_type {
            PropertyType::Adr => {
                if let ValueData::TextList(list) = &property.get_value().get_data() {
                    return format!("{}\n{} {}, {}\n{}", list[2], list[3], list[4], list[5], list[6]);
                }
                property.get_value().to_string()
            }
            _ => property.get_value().to_string(),
        }
    }
}

impl From<(PropertyType, &Vcard)> for PropertiesState {
    fn from((property_type, vcard): (PropertyType, &Vcard)) -> Self {
        Self {
            current: None,
            list: Default::default(),
            properties: vcard.get_properties_by_type(&property_type),
            property_type,
        }
    }
}

impl From<PropertyType> for PropertiesState {
    fn from(property_type: PropertyType) -> Self {
        Self {
            current: None,
            list: Default::default(),
            property_type,
            properties: Vec::new(),
        }
    }
}
