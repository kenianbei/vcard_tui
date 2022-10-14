use std::borrow::BorrowMut;

use vcard_parser::vcard::property::types::PropertyType;
use vcard_parser::vcard::Vcard;

use crate::state::contact::extra::ExtraPropertiesState;
use crate::state::contact::properties::PropertiesState;
use crate::state::contact::property::PropertyState;
use crate::state::list::StatefulList;
use crate::state::selected::{Select, Selected};

pub mod extra;
pub mod properties;
pub mod property;

#[derive(Clone)]
pub struct ContactState<'a> {
    pub properties_adr: PropertiesState,
    pub property_bday: PropertyState<'a>,
    pub properties_email: PropertiesState,
    pub properties_extra: ExtraPropertiesState,
    pub property_fn: PropertyState<'a>,
    pub properties_note: PropertiesState,
    pub properties_tel: PropertiesState,
    pub properties_url: PropertiesState,
    pub selected: Selected<ContactSelected>,
}

impl<'a> ContactState<'a> {
    pub fn get_selected_properties_state(&mut self) -> Option<&mut PropertiesState> {
        match self.selected.get() {
            ContactSelected::Email => Some(self.properties_email.borrow_mut()),
            ContactSelected::Tel => Some(self.properties_tel.borrow_mut()),
            ContactSelected::Adr => Some(self.properties_adr.borrow_mut()),
            ContactSelected::Url => Some(self.properties_url.borrow_mut()),
            ContactSelected::Note => Some(self.properties_note.borrow_mut()),
            _ => None,
        }
    }
    pub fn up(&mut self) {
        self.selected.prev();
        self.selected.prev();
        self.set_selected_properties_to_first();
    }
    pub fn down(&mut self) {
        self.selected.next();
        self.selected.next();
        self.set_selected_properties_to_first();
    }
    pub fn set_selected_properties_to_first(&mut self) {
        match self.selected.get() {
            ContactSelected::Email => self.properties_email.set_by_index(0),
            ContactSelected::Tel => self.properties_tel.set_by_index(0),
            ContactSelected::Adr => self.properties_adr.set_by_index(0),
            ContactSelected::Url => self.properties_url.set_by_index(0),
            ContactSelected::Note => self.properties_note.set_by_index(0),
            ContactSelected::Extra => self.properties_extra.set_by_index(0),
            _ => {}
        }
    }
}

impl<'a> Default for ContactState<'a> {
    fn default() -> Self {
        Self {
            properties_adr: PropertiesState::from(PropertyType::Adr),
            property_bday: PropertyState::from(PropertyType::BDay),
            properties_email: PropertiesState::from(PropertyType::Email),
            properties_extra: ExtraPropertiesState::default(),
            property_fn: PropertyState::from(PropertyType::Fn),
            properties_note: PropertiesState::from(PropertyType::Note),
            properties_tel: PropertiesState::from(PropertyType::Tel),
            properties_url: PropertiesState::from(PropertyType::Url),
            selected: Selected::from(ContactSelected::Fn),
        }
    }
}

impl<'a> From<&Vcard> for ContactState<'a> {
    fn from(vcard: &Vcard) -> Self {
        Self {
            properties_adr: PropertiesState::from((PropertyType::Adr, vcard)),
            property_bday: PropertyState::from((PropertyType::BDay, vcard)),
            properties_email: PropertiesState::from((PropertyType::Email, vcard)),
            properties_extra: ExtraPropertiesState::from(vcard),
            property_fn: PropertyState::from((PropertyType::Fn, vcard)),
            properties_note: PropertiesState::from((PropertyType::Note, vcard)),
            properties_tel: PropertiesState::from((PropertyType::Tel, vcard)),
            properties_url: PropertiesState::from((PropertyType::Url, vcard)),
            selected: Selected::from(ContactSelected::Fn),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ContactSelected {
    Fn,
    BDay,
    Email,
    Tel,
    Adr,
    Url,
    Note,
    Extra,
}

impl Select for Selected<ContactSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            ContactSelected::Fn => ContactSelected::Fn,
            ContactSelected::BDay => ContactSelected::Fn,
            ContactSelected::Email => ContactSelected::BDay,
            ContactSelected::Tel => ContactSelected::Email,
            ContactSelected::Adr => ContactSelected::Tel,
            ContactSelected::Url => ContactSelected::Adr,
            ContactSelected::Note => ContactSelected::Url,
            ContactSelected::Extra => ContactSelected::Note,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            ContactSelected::Fn => ContactSelected::BDay,
            ContactSelected::BDay => ContactSelected::Email,
            ContactSelected::Email => ContactSelected::Tel,
            ContactSelected::Tel => ContactSelected::Adr,
            ContactSelected::Adr => ContactSelected::Url,
            ContactSelected::Url => ContactSelected::Note,
            ContactSelected::Note => ContactSelected::Extra,
            ContactSelected::Extra => ContactSelected::Extra,
        })
    }
}
