use tui_textarea::TextArea;
use vcard_parser::traits::{HasParameters, HasValue};
use vcard_parser::vcard::parameter::Parameter;
use vcard_parser::vcard::property::property_url::PropertyUrlData;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::property::Property::PropertyUrl;
use vcard_parser::vcard::value::value_uri::ValueUriData;
use vcard_parser::vcard::value::Value::ValueUri;

use crate::state::selected::{Select, Selected};
use crate::tui::{HasTextArea, HasTypeParam};

#[derive(Clone)]
pub struct PropertyUrlState<'a> {
    pub property: Property,
    pub selected: Selected<PropertyUrlSelected>,
    pub textarea: Option<TextArea<'a>>,
}

impl<'a> Default for PropertyUrlState<'a> {
    fn default() -> Self {
        Self {
            property: PropertyUrl(PropertyUrlData::default()),
            selected: Selected::from(PropertyUrlSelected::Uri),
            textarea: None,
        }
    }
}

impl<'a> From<&Property> for PropertyUrlState<'a> {
    fn from(property: &Property) -> Self {
        Self {
            property: property.clone(),
            selected: Selected::from(PropertyUrlSelected::Uri),
            textarea: None,
        }
    }
}

impl<'a> HasTextArea<'a> for PropertyUrlState<'a> {
    fn textarea_value_get(&self) -> String {
        match self.selected.get() {
            PropertyUrlSelected::ParamType => self.type_param_get().get_value().to_string(),
            PropertyUrlSelected::Uri => self.property.get_value().to_string(),
            PropertyUrlSelected::Save => String::new(),
        }
    }
    fn textarea_value_set(&mut self, string: String) {
        match self.selected.get() {
            PropertyUrlSelected::ParamType => self.type_param_set(string),
            PropertyUrlSelected::Uri => {
                if let Ok(data) = ValueUriData::try_from(string.as_str()) {
                    self.property.set_value(ValueUri(data)).ok();
                }
            }
            PropertyUrlSelected::Save => {}
        }
    }
    fn textarea_get(&self) -> &Option<TextArea> {
        &self.textarea
    }
    fn textarea_set(&mut self, textarea: Option<TextArea<'a>>) {
        self.textarea = textarea
    }
}

impl<'a> HasTypeParam for PropertyUrlState<'a> {
    fn parameters_get(&self) -> Vec<Parameter> {
        self.property.get_parameters()
    }
    fn parameters_set(&mut self, parameters: Vec<Parameter>) {
        self.property.set_parameters(parameters)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum PropertyUrlSelected {
    ParamType,
    Uri,
    Save,
}

impl Select for Selected<PropertyUrlSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PropertyUrlSelected::ParamType => PropertyUrlSelected::ParamType,
            PropertyUrlSelected::Uri => PropertyUrlSelected::ParamType,
            PropertyUrlSelected::Save => PropertyUrlSelected::Uri,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PropertyUrlSelected::ParamType => PropertyUrlSelected::Uri,
            PropertyUrlSelected::Uri => PropertyUrlSelected::Save,
            PropertyUrlSelected::Save => PropertyUrlSelected::Save,
        })
    }
}

#[derive(Clone, Default)]
pub struct PropertyUrlValues {
    pub param_type: String,
    pub uri: String,
}
