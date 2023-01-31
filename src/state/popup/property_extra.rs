use tui_textarea::TextArea;
use vcard_parser::constants::ParameterName;
use vcard_parser::traits::{HasName, HasParameters, HasValue};
use vcard_parser::vcard::parameter::Parameter;
use vcard_parser::vcard::property::Property;

use crate::state::selected::{Select, Selected};
use crate::tui::HasTextArea;

#[derive(Clone)]
pub struct PropertyExtraState<'a> {
    pub name: String,
    pub parameters: String,
    pub pid: Option<Parameter>,
    pub value: String,
    pub selected: Selected<PropertyExtraSelected>,
    pub textarea: Option<TextArea<'a>>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum PropertyExtraSelected {
    Name,
    Value,
    Parameters,
    Save,
}

impl<'a> Default for PropertyExtraState<'a> {
    fn default() -> Self {
        Self {
            name: String::new(),
            parameters: String::new(),
            pid: None,
            value: String::new(),
            selected: Selected::from(PropertyExtraSelected::Name),
            textarea: None,
        }
    }
}

impl<'a> From<&Property> for PropertyExtraState<'a> {
    fn from(property: &Property) -> Self {
        Self {
            name: property.name().to_string(),
            parameters: property
                .get_parameters()
                .iter()
                .filter(|p| p.name() != ParameterName::PID)
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(""),
            pid: property.get_parameters().iter().find(|p| p.name() == ParameterName::PID).cloned(),
            value: property.get_value().to_string(),
            selected: Selected::from(PropertyExtraSelected::Name),
            textarea: None,
        }
    }
}

impl<'a> HasTextArea<'a> for PropertyExtraState<'a> {
    fn textarea_value_get(&self) -> String {
        match self.selected.get() {
            PropertyExtraSelected::Name => self.name.to_string(),
            PropertyExtraSelected::Value => self.value.to_string(),
            PropertyExtraSelected::Parameters => self.parameters.to_string(),
            PropertyExtraSelected::Save => String::new(),
        }
    }
    fn textarea_value_set(&mut self, string: String) {
        match self.selected.get() {
            PropertyExtraSelected::Name => self.name = string,
            PropertyExtraSelected::Value => self.value = string,
            PropertyExtraSelected::Parameters => self.parameters = string,
            PropertyExtraSelected::Save => {}
        }
    }
    fn textarea_get(&self) -> &Option<TextArea> {
        &self.textarea
    }
    fn textarea_set(&mut self, textarea: Option<TextArea<'a>>) {
        self.textarea = textarea
    }
}

impl Select for Selected<PropertyExtraSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PropertyExtraSelected::Name => PropertyExtraSelected::Name,
            PropertyExtraSelected::Value => PropertyExtraSelected::Name,
            PropertyExtraSelected::Parameters => PropertyExtraSelected::Value,
            PropertyExtraSelected::Save => PropertyExtraSelected::Parameters,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PropertyExtraSelected::Name => PropertyExtraSelected::Value,
            PropertyExtraSelected::Value => PropertyExtraSelected::Parameters,
            PropertyExtraSelected::Parameters => PropertyExtraSelected::Save,
            PropertyExtraSelected::Save => PropertyExtraSelected::Save,
        })
    }
}
