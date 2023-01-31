use tui_textarea::TextArea;
use vcard_parser::traits::HasValue;
use vcard_parser::vcard::property::property_bday::PropertyBDayData;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::property::Property::PropertyBDay;
use vcard_parser::vcard::value::value_date::ValueDateData;
use vcard_parser::vcard::value::Value::ValueDate;

use crate::state::selected::{Select, Selected};
use crate::tui::HasTextArea;

#[derive(Clone)]
pub struct PropertyBDayState<'a> {
    pub property: Property,
    pub selected: Selected<PropertyBDaySelected>,
    pub textarea: Option<TextArea<'a>>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum PropertyBDaySelected {
    Year,
    Month,
    Day,
    Save,
}

impl<'a> PropertyBDayState<'a> {
    pub fn value_date_data_get(&self) -> ValueDateData {
        if let ValueDate(data) = self.property.get_value() {
            data.clone()
        } else {
            ValueDateData::default()
        }
    }
}

impl<'a> Default for PropertyBDayState<'a> {
    fn default() -> Self {
        Self {
            property: PropertyBDay(PropertyBDayData::default()),
            selected: Selected::from(PropertyBDaySelected::Year),
            textarea: None,
        }
    }
}

impl<'a> From<&Property> for PropertyBDayState<'a> {
    fn from(property: &Property) -> Self {
        Self {
            property: property.clone(),
            selected: Selected::from(PropertyBDaySelected::Year),
            textarea: None,
        }
    }
}

impl<'a> HasTextArea<'a> for PropertyBDayState<'a> {
    fn textarea_value_get(&self) -> String {
        let data = self.value_date_data_get();
        match self.selected.get() {
            PropertyBDaySelected::Day => data.day.to_string(),
            PropertyBDaySelected::Month => data.month.to_string(),
            PropertyBDaySelected::Year => data.year.to_string(),
            PropertyBDaySelected::Save => String::new(),
        }
    }
    fn textarea_value_set(&mut self, string: String) {
        let value = string.parse::<i32>().unwrap_or(0);

        let mut data = self.value_date_data_get();
        match self.selected.get() {
            PropertyBDaySelected::Day => data.day = value as u8,
            PropertyBDaySelected::Month => data.month = value as u8,
            PropertyBDaySelected::Year => data.year = value,
            PropertyBDaySelected::Save => {}
        }

        self.property.set_value(ValueDate(data)).ok();
    }
    fn textarea_get(&self) -> &Option<TextArea> {
        &self.textarea
    }
    fn textarea_set(&mut self, textarea: Option<TextArea<'a>>) {
        self.textarea = textarea
    }
}

impl Select for Selected<PropertyBDaySelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PropertyBDaySelected::Year => PropertyBDaySelected::Year,
            PropertyBDaySelected::Month => PropertyBDaySelected::Year,
            PropertyBDaySelected::Day => PropertyBDaySelected::Month,
            PropertyBDaySelected::Save => PropertyBDaySelected::Day,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PropertyBDaySelected::Year => PropertyBDaySelected::Month,
            PropertyBDaySelected::Month => PropertyBDaySelected::Day,
            PropertyBDaySelected::Day => PropertyBDaySelected::Save,
            PropertyBDaySelected::Save => PropertyBDaySelected::Save,
        })
    }
}
