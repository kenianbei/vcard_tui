use crate::state::selected::{Select, Selected};

#[derive(Clone)]
pub struct ConfirmState {
    pub kind: ConfirmKind,
    pub selected: Selected<ConfirmSelected>,
}

impl Default for ConfirmState {
    fn default() -> Self {
        Self {
            kind: ConfirmKind::Quit,
            selected: Selected::from(ConfirmSelected::Confirm),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum ConfirmKind {
    Quit,
    Delete,
}

#[derive(Clone, Eq, PartialEq)]
pub enum ConfirmSelected {
    Cancel,
    Confirm,
}

impl Select for Selected<ConfirmSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            ConfirmSelected::Cancel => ConfirmSelected::Cancel,
            ConfirmSelected::Confirm => ConfirmSelected::Cancel,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            ConfirmSelected::Cancel => ConfirmSelected::Confirm,
            ConfirmSelected::Confirm => ConfirmSelected::Confirm,
        })
    }
}
