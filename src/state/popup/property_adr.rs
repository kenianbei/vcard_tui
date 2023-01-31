use tui_textarea::TextArea;
use vcard_parser::traits::{HasParameters, HasValue};
use vcard_parser::vcard::parameter::Parameter;
use vcard_parser::vcard::property::property_adr::PropertyAdrData;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::property::Property::PropertyAdr;
use vcard_parser::vcard::value::value_listcomponent::ValueListComponentData;
use vcard_parser::vcard::value::Value::ValueListComponent;

use crate::state::selected::{Select, Selected};
use crate::tui::{HasTextArea, HasTypeParam, HasValueListComponent};

#[derive(Clone)]
pub struct PropertyAdrState<'a> {
    pub property: Property,
    pub selected: Selected<PropertyAdrSelected>,
    pub textarea: Option<TextArea<'a>>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum PropertyAdrSelected {
    ParamType,
    Street,
    Locality,
    Region,
    Code,
    Country,
    Save,
}

impl<'a> Default for PropertyAdrState<'a> {
    fn default() -> Self {
        Self {
            property: PropertyAdr(PropertyAdrData::default()),
            selected: Selected::from(PropertyAdrSelected::Street),
            textarea: None,
        }
    }
}

impl<'a> From<&Property> for PropertyAdrState<'a> {
    fn from(property: &Property) -> Self {
        Self {
            property: property.clone(),
            selected: Selected::from(PropertyAdrSelected::Street),
            textarea: None,
        }
    }
}

impl<'a> HasTextArea<'a> for PropertyAdrState<'a> {
    fn textarea_value_get(&self) -> String {
        match self.selected.get() {
            PropertyAdrSelected::ParamType => self.type_param_get().get_value().to_string(),
            PropertyAdrSelected::Street => self.value_listcomponent_get_string(2),
            PropertyAdrSelected::Locality => self.value_listcomponent_get_string(3),
            PropertyAdrSelected::Region => self.value_listcomponent_get_string(4),
            PropertyAdrSelected::Code => self.value_listcomponent_get_string(5),
            PropertyAdrSelected::Country => self.value_listcomponent_get_string(6),
            PropertyAdrSelected::Save => String::new(),
        }
    }
    fn textarea_value_set(&mut self, string: String) {
        match self.selected.get() {
            PropertyAdrSelected::ParamType => self.type_param_set(string),
            PropertyAdrSelected::Street => self.value_listcomponent_set_string(2, string),
            PropertyAdrSelected::Locality => self.value_listcomponent_set_string(3, string),
            PropertyAdrSelected::Region => self.value_listcomponent_set_string(4, string),
            PropertyAdrSelected::Code => self.value_listcomponent_set_string(5, string),
            PropertyAdrSelected::Country => self.value_listcomponent_set_string(6, string),
            PropertyAdrSelected::Save => {}
        }
    }
    fn textarea_get(&self) -> &Option<TextArea> {
        &self.textarea
    }
    fn textarea_set(&mut self, textarea: Option<TextArea<'a>>) {
        self.textarea = textarea
    }
}

impl<'a> HasTypeParam for PropertyAdrState<'a> {
    fn parameters_get(&self) -> Vec<Parameter> {
        self.property.get_parameters()
    }
    fn parameters_set(&mut self, parameters: Vec<Parameter>) {
        self.property.set_parameters(parameters)
    }
}

impl<'a> HasValueListComponent for PropertyAdrState<'a> {
    fn value_data_get(&self) -> ValueListComponentData {
        if let ValueListComponent(data) = self.property.get_value() {
            data.clone()
        } else {
            ValueListComponentData::default()
        }
    }
    fn value_data_set(&mut self, data: ValueListComponentData) {
        self.property.set_value(ValueListComponent(data)).ok();
    }
}

impl Select for Selected<PropertyAdrSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PropertyAdrSelected::ParamType => PropertyAdrSelected::ParamType,
            PropertyAdrSelected::Street => PropertyAdrSelected::ParamType,
            PropertyAdrSelected::Locality => PropertyAdrSelected::Street,
            PropertyAdrSelected::Region => PropertyAdrSelected::Locality,
            PropertyAdrSelected::Code => PropertyAdrSelected::Region,
            PropertyAdrSelected::Country => PropertyAdrSelected::Code,
            PropertyAdrSelected::Save => PropertyAdrSelected::Country,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PropertyAdrSelected::ParamType => PropertyAdrSelected::Street,
            PropertyAdrSelected::Street => PropertyAdrSelected::Locality,
            PropertyAdrSelected::Locality => PropertyAdrSelected::Region,
            PropertyAdrSelected::Region => PropertyAdrSelected::Code,
            PropertyAdrSelected::Code => PropertyAdrSelected::Country,
            PropertyAdrSelected::Country => PropertyAdrSelected::Save,
            PropertyAdrSelected::Save => PropertyAdrSelected::Save,
        })
    }
}
