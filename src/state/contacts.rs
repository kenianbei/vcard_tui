use std::borrow::BorrowMut;
use std::fs::{read_to_string, write};

use ratatui::widgets::ListState;
use vcard_parser::constants::PropertyName;
use vcard_parser::error::VcardError;
use vcard_parser::parse_vcards;
use vcard_parser::traits::HasValue;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::Vcard;

use crate::state::contact::ContactState;
use crate::state::list::StatefulList;

#[derive(Clone, Default)]
pub struct ContactsState {
    pub current: ContactState,
    pub list: ListState,
    pub sort_asc: bool,
    pub vcards: Vec<Vcard>,
}

impl StatefulList<Vcard> for ContactsState {
    fn list(&mut self) -> &mut ListState {
        self.list.borrow_mut()
    }
    fn get(&self) -> Option<&Vcard> {
        if let Some(index) = self.list.selected() {
            self.vcards.get(index)
        } else {
            None
        }
    }
    fn set(&mut self, current: &Vcard) {
        let mut current = ContactState::from(current);
        current.selected = self.current.selected.clone();
        self.current = current;
    }
    fn vec(&self) -> Vec<Vcard> {
        self.vcards.clone()
    }
}

impl ContactsState {
    pub fn add(&mut self, vcard: Vcard) -> anyhow::Result<()> {
        self.vcards.push(vcard);
        self.sort_and_select(self.vcards.len() - 1);
        Ok(())
    }

    pub fn delete(&mut self) -> anyhow::Result<()> {
        if let Some(index) = self.list.selected() {
            self.vcards.remove(index);
            if self.vcards.is_empty() {
                self.current = ContactState::default();
            } else {
                self.sort_and_select(0);
            }
        }
        Ok(())
    }

    pub fn export(&mut self, path: &str) -> anyhow::Result<()> {
        let mut data = String::new();

        for vcard in &self.vcards {
            data.push_str(vcard.export().as_str())
        }

        write(path, data)?;

        Ok(())
    }

    pub fn import(&mut self, filename: &str) -> anyhow::Result<()> {
        let input = read_to_string(filename)?;
        match parse_vcards(input.as_str()) {
            Ok(vcards) => {
                self.vcards = vcards;
                self.sort();
                self.set_selected_index(0);
                Ok(())
            }
            Err(err) => Err(anyhow::Error::msg(match err {
                VcardError::ParseError(_) => err.parse_error(),
                _ => err.to_string(),
            })),
        }
    }

    pub fn update_property(&mut self, property: &Property) -> anyhow::Result<Property> {
        if let Some(index) = self.list.selected() {
            if let Some(vcard) = self.vcards.get_mut(index) {
                if let Ok(property) = vcard.set_property(property) {
                    self.set_selected_index(index);
                    return Ok(property);
                }
            }
        }
        Ok(property.clone())
    }

    pub fn remove_property(&mut self, property: &Property) -> anyhow::Result<()> {
        if let Some(index) = self.list.selected() {
            if let Some(vcard) = self.vcards.get_mut(index) {
                vcard.remove_property(property).ok();
                self.set_selected_index(index);
            }
        }
        Ok(())
    }

    pub fn sort(&mut self) {
        let cmp = |a: &Vcard, b: &Vcard| {
            let af = a.get_property_by_name(PropertyName::FN).unwrap();
            let bf = b.get_property_by_name(PropertyName::FN).unwrap();
            af.to_string().cmp(&bf.to_string())
        };
        if self.sort_asc {
            self.vcards.sort_by(|a, b| cmp(a, b).reverse())
        } else {
            self.vcards.sort_by(cmp)
        }
    }

    pub fn sort_and_select(&mut self, index: usize) {
        if let Some(vcard) = self.vcards.get(index).cloned() {
            self.sort();
            if let Some(a) = vcard.get_property_by_name(PropertyName::FN) {
                let position = self.vcards.iter().position(|v| {
                    if let Some(b) = v.get_property_by_name(PropertyName::FN) {
                        return a.get_value().to_string() == b.get_value().to_string();
                    }
                    false
                });
                if let Some(index) = position {
                    self.set_selected_index(index);
                }
            }
        }
    }
}
