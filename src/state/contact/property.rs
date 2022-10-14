use tui_textarea::TextArea;
use vcard_parser::uuid::Uuid;
use vcard_parser::vcard::property::types::PropertyType;
use vcard_parser::vcard::Vcard;

use crate::tui::HasTextArea;
use crate::util::property_title;

#[derive(Clone)]
pub struct PropertyState<'a> {
    pub property_type: PropertyType,
    pub textarea: Option<TextArea<'a>>,
    pub title: String,
    pub uuid: Option<Uuid>,
    pub value: String,
}

impl<'a> PropertyState<'a> {
    pub fn to_property_string(&self) -> String {
        format!("{}:{}", self.property_type, self.value)
    }
}

impl<'a> From<(PropertyType, &Vcard)> for PropertyState<'a> {
    fn from((property_type, vcard): (PropertyType, &Vcard)) -> Self {
        if let Some(property) = vcard.get_property_by_type(&property_type) {
            return PropertyState {
                property_type: property_type.clone(),
                textarea: None,
                title: property_title(&property_type).to_string(),
                uuid: Some(property.get_uuid()),
                value: property.get_value().to_string(),
            };
        } else {
            PropertyState {
                property_type: property_type.clone(),
                textarea: None,
                title: property_title(&property_type).to_string(),
                uuid: None,
                value: String::new(),
            }
        }
    }
}

impl<'a> From<PropertyType> for PropertyState<'a> {
    fn from(property_type: PropertyType) -> Self {
        PropertyState {
            property_type: property_type.clone(),
            textarea: None,
            title: property_title(&property_type).to_string(),
            uuid: None,
            value: String::new(),
        }
    }
}

impl<'a> HasTextArea<'a> for PropertyState<'a> {
    fn value_get(&self) -> String {
        self.value.to_string()
    }
    fn value_set(&mut self, string: String) {
        if self.property_type == PropertyType::Fn && string.is_empty() {
            return;
        }
        self.value = string
    }
    fn textarea_get(&self) -> &Option<TextArea> {
        &self.textarea
    }
    fn textarea_set(&mut self, textarea: Option<TextArea<'a>>) {
        self.textarea = textarea
    }
}
