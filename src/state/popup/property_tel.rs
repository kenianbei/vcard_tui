use tui_textarea::TextArea;
use vcard_parser::traits::{HasParameters, HasValue};
use vcard_parser::vcard::parameter::Parameter;
use vcard_parser::vcard::property::property_tel::PropertyTelData;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::property::Property::PropertyTel;
use vcard_parser::vcard::value::value_text::ValueTextData;
use vcard_parser::vcard::value::Value::ValueText;

use crate::state::selected::{Select, Selected};
use crate::tui::{HasTextArea, HasTypeParam};

#[derive(Clone)]
pub struct PropertyTelState<'a> {
    pub property: Property,
    pub selected: Selected<PropertyTelSelected>,
    pub textarea: Option<TextArea<'a>>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum PropertyTelSelected {
    ParamType,
    Tel,
    Save,
}

#[derive(Clone, Default)]
pub struct PropertyTelValues {
    pub param_type: String,
    pub tel: String,
}

impl<'a> Default for PropertyTelState<'a> {
    fn default() -> Self {
        Self {
            property: PropertyTel(PropertyTelData::default()),
            selected: Selected::from(PropertyTelSelected::Tel),
            textarea: None,
        }
    }
}

impl<'a> From<&Property> for PropertyTelState<'a> {
    fn from(property: &Property) -> Self {
        Self {
            property: property.clone(),
            selected: Selected::from(PropertyTelSelected::Tel),
            textarea: None,
        }
    }
}

impl<'a> HasTextArea<'a> for PropertyTelState<'a> {
    fn textarea_value_get(&self) -> String {
        match self.selected.get() {
            PropertyTelSelected::ParamType => self.type_param_get().get_value().to_string(),
            PropertyTelSelected::Tel => self.property.get_value().to_string(),
            PropertyTelSelected::Save => String::new(),
        }
    }
    fn textarea_value_set(&mut self, string: String) {
        match self.selected.get() {
            PropertyTelSelected::ParamType => self.type_param_set(string),
            PropertyTelSelected::Tel => {
                self.property.set_value(ValueText(ValueTextData::from(string.as_str()))).ok();
            }
            PropertyTelSelected::Save => {}
        }
    }
    fn textarea_get(&self) -> &Option<TextArea> {
        &self.textarea
    }
    fn textarea_set(&mut self, textarea: Option<TextArea<'a>>) {
        self.textarea = textarea
    }
}

impl<'a> HasTypeParam for PropertyTelState<'a> {
    fn parameters_get(&self) -> Vec<Parameter> {
        self.property.get_parameters()
    }
    fn parameters_set(&mut self, parameters: Vec<Parameter>) {
        self.property.set_parameters(parameters)
    }
}

impl Select for Selected<PropertyTelSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PropertyTelSelected::ParamType => PropertyTelSelected::ParamType,
            PropertyTelSelected::Tel => PropertyTelSelected::ParamType,
            PropertyTelSelected::Save => PropertyTelSelected::Tel,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PropertyTelSelected::ParamType => PropertyTelSelected::Tel,
            PropertyTelSelected::Tel => PropertyTelSelected::Save,
            PropertyTelSelected::Save => PropertyTelSelected::Save,
        })
    }
}
