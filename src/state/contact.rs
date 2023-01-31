use std::borrow::BorrowMut;

use vcard_parser::constants::PropertyName;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::Vcard;

use crate::state::extra::ExtraPropertiesState;
use crate::state::list::StatefulList;
use crate::state::properties::PropertiesState;
use crate::state::selected::{Select, Selected};

#[derive(Clone)]
pub struct ContactState {
    pub properties_adr: PropertiesState,
    pub property_bday: Option<Property>,
    pub properties_email: PropertiesState,
    pub properties_extra: ExtraPropertiesState,
    pub property_n: Option<Property>,
    pub properties_note: PropertiesState,
    pub properties_tel: PropertiesState,
    pub properties_url: PropertiesState,
    pub selected: Selected<ContactSelected>,
}

impl ContactState {
    pub fn get_selected_property(&mut self) -> Option<&Property> {
        match self.selected.get() {
            ContactSelected::Adr => self.properties_adr.current.as_ref(),
            ContactSelected::BDay => self.property_bday.as_ref(),
            ContactSelected::Email => self.properties_email.current.as_ref(),
            ContactSelected::Extra => self.properties_extra.current.as_ref(),
            ContactSelected::N => self.property_n.as_ref(),
            ContactSelected::Note => self.properties_note.current.as_ref(),
            ContactSelected::Tel => self.properties_tel.current.as_ref(),
            ContactSelected::Url => self.properties_url.current.as_ref(),
        }
    }
    pub fn get_selected_properties_state(&mut self) -> Option<&mut PropertiesState> {
        match self.selected.get() {
            ContactSelected::Adr => Some(self.properties_adr.borrow_mut()),
            ContactSelected::Email => Some(self.properties_email.borrow_mut()),
            ContactSelected::Note => Some(self.properties_note.borrow_mut()),
            ContactSelected::Tel => Some(self.properties_tel.borrow_mut()),
            ContactSelected::Url => Some(self.properties_url.borrow_mut()),
            _ => None,
        }
    }
    pub fn up(&mut self) {
        match self.selected.get() {
            ContactSelected::BDay | ContactSelected::N => {
                self.selected.prev();
                self.selected.prev();
                self.set_selected_properties_to_first();
            }
            ContactSelected::Extra => {
                if self.properties_extra.properties.len() <= 1 {
                    self.selected.prev();
                    self.selected.prev();
                    self.set_selected_properties_to_first();
                } else {
                    self.properties_extra.prev()
                }
            }
            _ => {
                if let Some(s) = self.get_selected_properties_state() {
                    if s.properties.len() <= 1 {
                        self.selected.prev();
                        self.selected.prev();
                        self.set_selected_properties_to_first();
                    } else {
                        s.prev()
                    }
                }
            }
        }
    }
    pub fn down(&mut self) {
        match self.selected.get() {
            ContactSelected::BDay | ContactSelected::N => {
                self.selected.next();
                self.selected.next();
                self.set_selected_properties_to_first();
            }
            ContactSelected::Extra => {
                if self.properties_extra.properties.len() <= 1 {
                    self.selected.next();
                    self.selected.next();
                    self.set_selected_properties_to_first();
                } else {
                    self.properties_extra.next()
                }
            }
            _ => {
                if let Some(s) = self.get_selected_properties_state() {
                    if s.properties.len() <= 1 {
                        self.selected.next();
                        self.selected.next();
                        self.set_selected_properties_to_first();
                    } else {
                        s.next()
                    }
                }
            }
        }
    }
    pub fn set_selected_properties_to_first(&mut self) {
        match self.selected.get() {
            ContactSelected::Email => self.properties_email.set_selected_index(0),
            ContactSelected::Tel => self.properties_tel.set_selected_index(0),
            ContactSelected::Adr => self.properties_adr.set_selected_index(0),
            ContactSelected::Url => self.properties_url.set_selected_index(0),
            ContactSelected::Note => self.properties_note.set_selected_index(0),
            ContactSelected::Extra => self.properties_extra.set_selected_index(0),
            _ => {}
        }
    }
}

impl Default for ContactState {
    fn default() -> Self {
        Self {
            properties_adr: PropertiesState::from(PropertyName::ADR),
            property_bday: None,
            properties_email: PropertiesState::from(PropertyName::EMAIL),
            properties_extra: ExtraPropertiesState::default(),
            property_n: None,
            properties_note: PropertiesState::from(PropertyName::NOTE),
            properties_tel: PropertiesState::from(PropertyName::TEL),
            properties_url: PropertiesState::from(PropertyName::URL),
            selected: Selected::from(ContactSelected::N),
        }
    }
}

impl From<&Vcard> for ContactState {
    fn from(vcard: &Vcard) -> Self {
        Self {
            properties_adr: PropertiesState::from((PropertyName::ADR, vcard)),
            property_bday: vcard.get_property_by_name(PropertyName::BDAY),
            properties_email: PropertiesState::from((PropertyName::EMAIL, vcard)),
            properties_extra: ExtraPropertiesState::from(vcard),
            property_n: vcard.get_property_by_name(PropertyName::N),
            properties_note: PropertiesState::from((PropertyName::NOTE, vcard)),
            properties_tel: PropertiesState::from((PropertyName::TEL, vcard)),
            properties_url: PropertiesState::from((PropertyName::URL, vcard)),
            selected: Selected::from(ContactSelected::N),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ContactSelected {
    N,
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
            ContactSelected::N => ContactSelected::N,
            ContactSelected::BDay => ContactSelected::N,
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
            ContactSelected::N => ContactSelected::BDay,
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
