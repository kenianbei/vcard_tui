use tui_textarea::TextArea;
use vcard_parser::traits::{HasParameters, HasValue};
use vcard_parser::vcard::parameter::Parameter;
use vcard_parser::vcard::property::property_email::PropertyEmailData;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::property::Property::PropertyEmail;
use vcard_parser::vcard::value::value_text::ValueTextData;
use vcard_parser::vcard::value::Value::ValueText;

use crate::state::selected::{Select, Selected};
use crate::tui::{HasTextArea, HasTypeParam};

#[derive(Clone)]
pub struct PropertyEmailState<'a> {
    pub property: Property,
    pub selected: Selected<PropertyEmailSelected>,
    pub textarea: Option<TextArea<'a>>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum PropertyEmailSelected {
    ParamType,
    Email,
    Save,
}

impl<'a> Default for PropertyEmailState<'a> {
    fn default() -> Self {
        Self {
            property: PropertyEmail(PropertyEmailData::default()),
            selected: Selected::from(PropertyEmailSelected::Email),
            textarea: None,
        }
    }
}

impl<'a> From<&Property> for PropertyEmailState<'a> {
    fn from(property: &Property) -> Self {
        Self {
            property: property.clone(),
            selected: Selected::from(PropertyEmailSelected::Email),
            textarea: None,
        }
    }
}

impl<'a> HasTypeParam for PropertyEmailState<'a> {
    fn parameters_get(&self) -> Vec<Parameter> {
        self.property.get_parameters()
    }
    fn parameters_set(&mut self, parameters: Vec<Parameter>) {
        self.property.set_parameters(parameters)
    }
}

impl<'a> HasTextArea<'a> for PropertyEmailState<'a> {
    fn textarea_value_get(&self) -> String {
        match self.selected.get() {
            PropertyEmailSelected::ParamType => self.type_param_get().get_value().to_string(),
            PropertyEmailSelected::Email => self.property.get_value().to_string(),
            PropertyEmailSelected::Save => String::new(),
        }
    }
    fn textarea_value_set(&mut self, string: String) {
        match self.selected.get() {
            PropertyEmailSelected::ParamType => self.type_param_set(string),
            PropertyEmailSelected::Email => {
                self.property.set_value(ValueText(ValueTextData::from(string.as_str()))).ok();
            }
            PropertyEmailSelected::Save => {}
        }
    }
    fn textarea_get(&self) -> &Option<TextArea> {
        &self.textarea
    }
    fn textarea_set(&mut self, textarea: Option<TextArea<'a>>) {
        self.textarea = textarea
    }
}

impl Select for Selected<PropertyEmailSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PropertyEmailSelected::ParamType => PropertyEmailSelected::ParamType,
            PropertyEmailSelected::Email => PropertyEmailSelected::ParamType,
            PropertyEmailSelected::Save => PropertyEmailSelected::Email,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PropertyEmailSelected::ParamType => PropertyEmailSelected::Email,
            PropertyEmailSelected::Email => PropertyEmailSelected::Save,
            PropertyEmailSelected::Save => PropertyEmailSelected::Save,
        })
    }
}
