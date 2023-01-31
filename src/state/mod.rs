use crate::state::contacts::ContactsState;
use crate::state::popup::PopupState;
use crate::state::selected::{Select, Selected};

pub mod contact;
pub mod contacts;
pub mod extra;
pub mod files;
pub mod list;
pub mod popup;
pub mod properties;
pub mod selected;

#[derive(Clone)]
pub struct State<'a> {
    pub contacts: ContactsState,
    pub popup: Option<PopupState<'a>>,
    pub render: bool,
    pub selected: Selected<StateSelected>,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self {
            contacts: ContactsState::default(),
            popup: None,
            render: true,
            selected: Selected::from(StateSelected::Contacts),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum StateSelected {
    Contacts,
    Contact,
}

impl Select for Selected<StateSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            StateSelected::Contacts => StateSelected::Contacts,
            StateSelected::Contact => StateSelected::Contacts,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            StateSelected::Contacts => StateSelected::Contact,
            StateSelected::Contact => StateSelected::Contact,
        })
    }
}
