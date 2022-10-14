use tui_textarea::TextArea;
use vcard_parser::uuid::Uuid;
use vcard_parser::vcard::parameter::types::ParameterType;
use vcard_parser::vcard::property::Property;

use crate::state::selected::{Select, Selected};
use crate::tui::HasTextArea;

#[derive(Clone)]
pub struct PropertyUrlState<'a> {
    pub selected: Selected<PopupUrlSelected>,
    pub textarea: Option<TextArea<'a>>,
    pub values: PopupUrlValues,
    pub uuid: Option<Uuid>,
}

impl<'a> PropertyUrlState<'a> {
    pub fn to_property_string(&self) -> String {
        if !self.values.param_type.is_empty() {
            format!("URL;TYPE={}:{}", self.values.param_type, self.values.uri)
        } else {
            format!("URL:{}", self.values.uri)
        }
    }
}

impl<'a> Default for PropertyUrlState<'a> {
    fn default() -> Self {
        Self {
            selected: Selected::from(PopupUrlSelected::Uri),
            textarea: None,
            uuid: None,
            values: PopupUrlValues::default(),
        }
    }
}

impl<'a> From<&Property> for PropertyUrlState<'a> {
    fn from(property: &Property) -> Self {
        let mut values = PopupUrlValues {
            param_type: String::new(),
            uri: property.get_value().to_string(),
        };

        if let Some(parameter) = property.get_parameters().iter().find(|p| p.is_type(ParameterType::Type)) {
            values.param_type = parameter.get_value().to_string()
        }

        Self {
            selected: Selected::from(PopupUrlSelected::Uri),
            textarea: None,
            uuid: Some(property.get_uuid()),
            values,
        }
    }
}

impl<'a> HasTextArea<'a> for PropertyUrlState<'a> {
    fn value_get(&self) -> String {
        match self.selected.get() {
            PopupUrlSelected::ParamType => self.values.param_type.to_string(),
            PopupUrlSelected::Uri => self.values.uri.to_string(),
            PopupUrlSelected::Save => String::new(),
        }
    }
    fn value_set(&mut self, string: String) {
        match self.selected.get() {
            PopupUrlSelected::ParamType => self.values.param_type = string,
            PopupUrlSelected::Uri => self.values.uri = string,
            PopupUrlSelected::Save => {}
        }
    }
    fn textarea_get(&self) -> &Option<TextArea> {
        &self.textarea
    }
    fn textarea_set(&mut self, textarea: Option<TextArea<'a>>) {
        self.textarea = textarea
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum PopupUrlSelected {
    ParamType,
    Uri,
    Save,
}

impl Select for Selected<PopupUrlSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PopupUrlSelected::ParamType => PopupUrlSelected::ParamType,
            PopupUrlSelected::Uri => PopupUrlSelected::ParamType,
            PopupUrlSelected::Save => PopupUrlSelected::Uri,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PopupUrlSelected::ParamType => PopupUrlSelected::Uri,
            PopupUrlSelected::Uri => PopupUrlSelected::Save,
            PopupUrlSelected::Save => PopupUrlSelected::Save,
        })
    }
}

#[derive(Clone, Default)]
pub struct PopupUrlValues {
    pub param_type: String,
    pub uri: String,
}
