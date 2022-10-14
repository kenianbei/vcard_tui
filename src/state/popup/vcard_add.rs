use tui_textarea::TextArea;

use crate::state::selected::{Select, Selected};

#[derive(Clone)]
pub struct VcardAddState<'a> {
    pub selected: Selected<PopupVcardAddSelected>,
    pub textarea: TextArea<'a>,
}

impl<'a> Default for VcardAddState<'a> {
    fn default() -> Self {
        Self {
            selected: Selected::from(PopupVcardAddSelected::TextArea),
            textarea: TextArea::default(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum PopupVcardAddSelected {
    TextArea,
    Create,
}

impl Select for Selected<PopupVcardAddSelected> {
    fn prev(&mut self) {
        self.set(match self.get() {
            PopupVcardAddSelected::TextArea => PopupVcardAddSelected::TextArea,
            PopupVcardAddSelected::Create => PopupVcardAddSelected::TextArea,
        })
    }
    fn next(&mut self) {
        self.set(match self.get() {
            PopupVcardAddSelected::TextArea => PopupVcardAddSelected::Create,
            PopupVcardAddSelected::Create => PopupVcardAddSelected::Create,
        })
    }
}
