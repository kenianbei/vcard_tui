use tui_textarea::TextArea;
use vcard_parser::uuid::Uuid;
use vcard_parser::vcard::parameter::types::ParameterType;
use vcard_parser::vcard::property::Property;

use crate::state::selected::{Select, Selected};
use crate::tui::HasTextArea;

#[derive(Clone)]
pub struct PropertyEmailState<'a> {
    pub selected: Selected<PopupEmailSelected>,
    pub textarea: Option<TextArea<'a>>,
    pub values: PopupEmailValues,
    pub uuid: Option<Uuid>,
}

impl<'a> PropertyEmailState<'a> {
    pub fn to_property_string(&self) -> String {
        if !self.values.param_type.is_empty() {
            format!("EMAIL;TYPE={}:{}", self.values.param_type, self.values.email)
        } else {
            format!("EMAIL:{}", self.values.email)
        }
    }
}

impl<'a> Default for PropertyEmailState<'a> {
    fn default() -> Self {
        Self {
            selected: Selected::from(PopupEmailSelected::Email),
            textarea: None,
            uuid: None,
            values: PopupEmailValues::default(),
        }
    }
}

impl<'a> From<&Property> for PropertyEmailState<'a> {
    fn from(property: &Property) -> Self {
        let mut values = PopupEmailValues {
            param_type: String::new(),
            email: property.get_value().to_string(),
        };

        if let Some(parameter) = property.get_parameters().iter().find(|p| p.is_type(ParameterType::Type)) {
            values.param_type = parameter.get_value().to_string()
        }

        Self {
            selected: Selected::from(PopupEmailSelected::Email),
            textarea: None,
            uuid: Some(property.get_uuid()),
            values,
        }
    }
}

impl<'a> HasTextArea<'a> for PropertyEmailState<'a> {
    fn value_get(&self) -> String {
        match self.selected.get() {
            PopupEmailSelected::ParamType => self.values.param_type.to_string(),
            PopupEmailSelected::Email => self.values.email.to_string(),
            PopupEmailSelected::Save => String::new(),
        }
    }
    fn value_set(&mut self, string: String) {
        match self.selected.get() {
            PopupEmailSelected::ParamType => self.values.param_type = string,
            PopupEmailSelected::Email => self.values.email = string,
            PopupEmailSelected::Save => {}
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
pub enum PopupEmailSelected {
    ParamType,
    Email,
    Save,
}

impl Select for Selected<PopupEmailSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PopupEmailSelected::ParamType => PopupEmailSelected::ParamType,
            PopupEmailSelected::Email => PopupEmailSelected::ParamType,
            PopupEmailSelected::Save => PopupEmailSelected::Email,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PopupEmailSelected::ParamType => PopupEmailSelected::Email,
            PopupEmailSelected::Email => PopupEmailSelected::Save,
            PopupEmailSelected::Save => PopupEmailSelected::Save,
        })
    }
}

#[derive(Clone, Default)]
pub struct PopupEmailValues {
    pub param_type: String,
    pub email: String,
}
