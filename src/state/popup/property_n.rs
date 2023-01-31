use tui_textarea::TextArea;
use vcard_parser::traits::HasValue;
use vcard_parser::vcard::property::property_n::PropertyNData;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::property::Property::PropertyN;
use vcard_parser::vcard::value::value_listcomponent::ValueListComponentData;
use vcard_parser::vcard::value::Value::ValueListComponent;

use crate::state::selected::{Select, Selected};
use crate::tui::{HasTextArea, HasValueListComponent};

#[derive(Clone)]
pub struct PropertyNState<'a> {
    pub property: Property,
    pub selected: Selected<PropertyNSelected>,
    pub textarea: Option<TextArea<'a>>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum PropertyNSelected {
    Family,
    Given,
    Additional,
    Save,
}

impl<'a> Default for PropertyNState<'a> {
    fn default() -> Self {
        Self {
            property: PropertyN(PropertyNData::default()),
            selected: Selected::from(PropertyNSelected::Given),
            textarea: None,
        }
    }
}

impl<'a> From<&Property> for PropertyNState<'a> {
    fn from(property: &Property) -> Self {
        Self {
            property: property.clone(),
            selected: Selected::from(PropertyNSelected::Given),
            textarea: None,
        }
    }
}

impl<'a> HasTextArea<'a> for PropertyNState<'a> {
    fn textarea_value_get(&self) -> String {
        match self.selected.get() {
            PropertyNSelected::Given => self.value_listcomponent_get_string(1),
            PropertyNSelected::Family => self.value_listcomponent_get_string(0),
            PropertyNSelected::Additional => self.value_listcomponent_get_string(2),
            PropertyNSelected::Save => String::new(),
        }
    }
    fn textarea_value_set(&mut self, string: String) {
        match self.selected.get() {
            PropertyNSelected::Given => self.value_listcomponent_set_string(1, string),
            PropertyNSelected::Family => self.value_listcomponent_set_string(0, string),
            PropertyNSelected::Additional => self.value_listcomponent_set_string(2, string),
            PropertyNSelected::Save => {}
        }
    }
    fn textarea_get(&self) -> &Option<TextArea> {
        &self.textarea
    }
    fn textarea_set(&mut self, textarea: Option<TextArea<'a>>) {
        self.textarea = textarea
    }
}

impl<'a> HasValueListComponent for PropertyNState<'a> {
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

impl Select for Selected<PropertyNSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PropertyNSelected::Given => PropertyNSelected::Given,
            PropertyNSelected::Family => PropertyNSelected::Given,
            PropertyNSelected::Additional => PropertyNSelected::Family,
            PropertyNSelected::Save => PropertyNSelected::Additional,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PropertyNSelected::Given => PropertyNSelected::Family,
            PropertyNSelected::Family => PropertyNSelected::Additional,
            PropertyNSelected::Additional => PropertyNSelected::Save,
            PropertyNSelected::Save => PropertyNSelected::Save,
        })
    }
}
