use tui_textarea::TextArea;
use vcard_parser::uuid::Uuid;
use vcard_parser::vcard::parameter::types::ParameterType;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::values::data::ValueData;

use crate::state::selected::{Select, Selected};
use crate::tui::HasTextArea;

#[derive(Clone)]
pub struct PropertyAdrState<'a> {
    pub selected: Selected<PopupAdrSelected>,
    pub textarea: Option<TextArea<'a>>,
    pub uuid: Option<Uuid>,
    pub values: PopupAdrValues,
}

impl<'a> PropertyAdrState<'a> {
    pub fn to_property_string(&self) -> String {
        let values = format!(";;{};{};{};{};{}", self.values.street, self.values.locality, self.values.region, self.values.code, self.values.country);

        let mut string = format!("ADR:{}", values);
        if !self.values.param_type.is_empty() {
            string = format!("ADR;TYPE={}:{}", self.values.param_type, values);
        }

        string
    }
}

impl<'a> Default for PropertyAdrState<'a> {
    fn default() -> Self {
        Self {
            selected: Selected::from(PopupAdrSelected::Street),
            textarea: None,
            uuid: None,
            values: PopupAdrValues::default(),
        }
    }
}

impl<'a> From<&Property> for PropertyAdrState<'a> {
    fn from(property: &Property) -> Self {
        let mut values = PopupAdrValues::default();

        if let ValueData::TextList(v) = property.get_value().get_data() {
            if let Some(street) = v.get(2) {
                values.street = street.clone()
            }
            if let Some(locality) = v.get(3) {
                values.locality = locality.clone()
            }
            if let Some(region) = v.get(4) {
                values.region = region.clone()
            }
            if let Some(code) = v.get(5) {
                values.code = code.clone()
            }
            if let Some(country) = v.get(6) {
                values.country = country.clone()
            }
        }

        if let Some(parameter) = property.get_parameters().iter().find(|p| p.is_type(ParameterType::Type)) {
            values.param_type = parameter.get_value().to_string()
        }

        Self {
            selected: Selected::from(PopupAdrSelected::Street),
            textarea: None,
            uuid: Some(property.get_uuid()),
            values,
        }
    }
}

impl<'a> HasTextArea<'a> for PropertyAdrState<'a> {
    fn value_get(&self) -> String {
        match self.selected.get() {
            PopupAdrSelected::ParamType => self.values.param_type.to_string(),
            PopupAdrSelected::Street => self.values.street.to_string(),
            PopupAdrSelected::Locality => self.values.locality.to_string(),
            PopupAdrSelected::Region => self.values.region.to_string(),
            PopupAdrSelected::Code => self.values.code.to_string(),
            PopupAdrSelected::Country => self.values.country.to_string(),
            PopupAdrSelected::Save => String::new(),
        }
    }
    fn value_set(&mut self, string: String) {
        match self.selected.get() {
            PopupAdrSelected::ParamType => self.values.param_type = string,
            PopupAdrSelected::Street => self.values.street = string,
            PopupAdrSelected::Locality => self.values.locality = string,
            PopupAdrSelected::Region => self.values.region = string,
            PopupAdrSelected::Code => self.values.code = string,
            PopupAdrSelected::Country => self.values.country = string,
            PopupAdrSelected::Save => {}
        }
    }
    fn textarea_get(&self) -> &Option<TextArea> {
        &self.textarea
    }
    fn textarea_set(&mut self, textarea: Option<TextArea<'a>>) {
        self.textarea = textarea
    }
}

#[derive(Clone, Default)]
pub struct PopupAdrValues {
    pub param_type: String,
    pub street: String,
    pub locality: String,
    pub region: String,
    pub code: String,
    pub country: String,
}

#[derive(Clone, Eq, PartialEq)]
pub enum PopupAdrSelected {
    ParamType,
    Street,
    Locality,
    Region,
    Code,
    Country,
    Save,
}

impl Select for Selected<PopupAdrSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PopupAdrSelected::ParamType => PopupAdrSelected::ParamType,
            PopupAdrSelected::Street => PopupAdrSelected::ParamType,
            PopupAdrSelected::Locality => PopupAdrSelected::Street,
            PopupAdrSelected::Region => PopupAdrSelected::Locality,
            PopupAdrSelected::Code => PopupAdrSelected::Region,
            PopupAdrSelected::Country => PopupAdrSelected::Code,
            PopupAdrSelected::Save => PopupAdrSelected::Country,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PopupAdrSelected::ParamType => PopupAdrSelected::Street,
            PopupAdrSelected::Street => PopupAdrSelected::Locality,
            PopupAdrSelected::Locality => PopupAdrSelected::Region,
            PopupAdrSelected::Region => PopupAdrSelected::Code,
            PopupAdrSelected::Code => PopupAdrSelected::Country,
            PopupAdrSelected::Country => PopupAdrSelected::Save,
            PopupAdrSelected::Save => PopupAdrSelected::Save,
        })
    }
}
