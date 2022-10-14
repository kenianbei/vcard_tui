use std::borrow::BorrowMut;
use std::fs::{read_to_string, write};

use tui::widgets::ListState;
use vcard_parser::parse_to_vcards;
use vcard_parser::uuid::Uuid;
use vcard_parser::vcard::property::types::PropertyType;
use vcard_parser::vcard::Vcard;

use crate::state::contact::ContactState;
use crate::state::list::StatefulList;

#[derive(Clone, Default)]
pub struct ContactsState<'a> {
    pub current: ContactState<'a>,
    pub list: ListState,
    pub sort_asc: bool,
    pub vcards: Vec<Vcard>,
}

impl<'a> StatefulList<Vcard> for ContactsState<'a> {
    fn list(&mut self) -> &mut ListState {
        self.list.borrow_mut()
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

impl<'a> ContactsState<'a> {
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
            data.push_str(vcard.to_string().as_str())
        }

        write(path, data)?;

        Ok(())
    }

    pub fn import(&mut self, filename: &str) -> anyhow::Result<()> {
        let input = read_to_string(filename)?;
        self.vcards = parse_to_vcards(input.as_str())?;
        self.sort();
        self.set_by_index(0);
        Ok(())
    }

    pub fn update_property(&mut self, uuid: Option<Uuid>, str: &str) -> anyhow::Result<()> {
        if let Some(index) = self.list.selected() {
            if let Some(vcard) = self.vcards.get_mut(index) {
                match uuid {
                    None => {
                        vcard.add_property(str)?;
                    }
                    Some(uuid) => {
                        vcard.update_property(uuid, str)?;
                    }
                }
                self.set_by_index(index);
            }
        }
        Ok(())
    }

    pub fn remove_property(&mut self, uuid: Uuid) -> anyhow::Result<()> {
        if let Some(index) = self.list.selected() {
            if let Some(vcard) = self.vcards.get_mut(index) {
                vcard.remove_property(uuid)?;
                self.set_by_index(index);
            }
        }
        Ok(())
    }

    pub fn sort(&mut self) {
        let cmp = |a: &Vcard, b: &Vcard| {
            let af = a.get_property_by_type(&PropertyType::Fn).unwrap();
            let bf = b.get_property_by_type(&PropertyType::Fn).unwrap();
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
            if let Some(a) = vcard.get_property_by_type(&PropertyType::Fn) {
                let position = self.vcards.iter().position(|v| {
                    if let Some(b) = v.get_property_by_type(&PropertyType::Fn) {
                        return a.get_value().to_string() == b.get_value().to_string();
                    }
                    false
                });
                if let Some(index) = position {
                    self.set_by_index(index);
                }
            }
        }
    }
}
