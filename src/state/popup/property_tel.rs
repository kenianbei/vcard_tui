use tui_textarea::TextArea;
use vcard_parser::uuid::Uuid;
use vcard_parser::vcard::parameter::types::ParameterType;
use vcard_parser::vcard::property::Property;

use crate::state::selected::{Select, Selected};
use crate::tui::HasTextArea;

#[derive(Clone)]
pub struct PropertyTelState<'a> {
    pub selected: Selected<PopupTelSelected>,
    pub textarea: Option<TextArea<'a>>,
    pub values: PopupTelValues,
    pub uuid: Option<Uuid>,
}

impl<'a> PropertyTelState<'a> {
    pub fn to_property_string(&self) -> String {
        if !self.values.param_type.is_empty() {
            format!("TEL;TYPE={}:{}", self.values.param_type, self.values.tel)
        } else {
            format!("TEL:{}", self.values.tel)
        }
    }
}

impl<'a> Default for PropertyTelState<'a> {
    fn default() -> Self {
        Self {
            selected: Selected::from(PopupTelSelected::Tel),
            textarea: None,
            uuid: None,
            values: PopupTelValues::default(),
        }
    }
}

impl<'a> From<&Property> for PropertyTelState<'a> {
    fn from(property: &Property) -> Self {
        let mut values = PopupTelValues {
            param_type: String::new(),
            tel: property.get_value().to_string(),
        };

        if let Some(parameter) = property.get_parameters().iter().find(|p| p.is_type(ParameterType::Type)) {
            values.param_type = parameter.get_value().to_string()
        }

        Self {
            selected: Selected::from(PopupTelSelected::Tel),
            textarea: None,
            uuid: Some(property.get_uuid()),
            values,
        }
    }
}

impl<'a> HasTextArea<'a> for PropertyTelState<'a> {
    fn value_get(&self) -> String {
        match self.selected.get() {
            PopupTelSelected::ParamType => self.values.param_type.to_string(),
            PopupTelSelected::Tel => self.values.tel.to_string(),
            PopupTelSelected::Save => String::new(),
        }
    }
    fn value_set(&mut self, string: String) {
        match self.selected.get() {
            PopupTelSelected::ParamType => self.values.param_type = string,
            PopupTelSelected::Tel => self.values.tel = string,
            PopupTelSelected::Save => {}
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
pub enum PopupTelSelected {
    ParamType,
    Tel,
    Save,
}

impl Select for Selected<PopupTelSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PopupTelSelected::ParamType => PopupTelSelected::ParamType,
            PopupTelSelected::Tel => PopupTelSelected::ParamType,
            PopupTelSelected::Save => PopupTelSelected::Tel,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PopupTelSelected::ParamType => PopupTelSelected::Tel,
            PopupTelSelected::Tel => PopupTelSelected::Save,
            PopupTelSelected::Save => PopupTelSelected::Save,
        })
    }
}

#[derive(Clone, Default)]
pub struct PopupTelValues {
    pub param_type: String,
    pub tel: String,
}
