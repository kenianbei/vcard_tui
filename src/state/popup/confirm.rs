use crate::state::selected::{Select, Selected};

#[derive(Clone)]
pub struct ConfirmState {
    pub kind: PopupConfirmKind,
    pub selected: Selected<PopupConfirmSelected>,
}

impl Default for ConfirmState {
    fn default() -> Self {
        Self {
            kind: PopupConfirmKind::Quit,
            selected: Selected::from(PopupConfirmSelected::Confirm),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum PopupConfirmKind {
    Quit,
    Delete,
}

#[derive(Clone, Eq, PartialEq)]
pub enum PopupConfirmSelected {
    Cancel,
    Confirm,
}

impl Select for Selected<PopupConfirmSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PopupConfirmSelected::Cancel => PopupConfirmSelected::Cancel,
            PopupConfirmSelected::Confirm => PopupConfirmSelected::Cancel,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PopupConfirmSelected::Cancel => PopupConfirmSelected::Confirm,
            PopupConfirmSelected::Confirm => PopupConfirmSelected::Confirm,
        })
    }
}
