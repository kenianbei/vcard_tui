use vcard_parser::constants::PropertyName;
use vcard_parser::traits::HasName;
use vcard_parser::vcard::property::Property;

use crate::state::popup::confirm::ConfirmState;
use crate::state::popup::export::ExportState;
use crate::state::popup::import::ImportState;
use crate::state::popup::message::MessageState;
use crate::state::popup::property_adr::PropertyAdrState;
use crate::state::popup::property_bday::PropertyBDayState;
use crate::state::popup::property_email::PropertyEmailState;
use crate::state::popup::property_extra::PropertyExtraState;
use crate::state::popup::property_n::PropertyNState;
use crate::state::popup::property_note::PropertyNoteState;
use crate::state::popup::property_remove::PropertyRemoveState;
use crate::state::popup::property_tel::PropertyTelState;
use crate::state::popup::property_url::PropertyUrlState;
use crate::state::popup::vcard_add::VcardAddState;
use crate::PopupState::PropertyExtra;

pub mod confirm;
pub mod export;
pub mod import;
pub mod message;
pub mod property_adr;
pub mod property_bday;
pub mod property_email;
pub mod property_extra;
pub mod property_n;
pub mod property_note;
pub mod property_remove;
pub mod property_tel;
pub mod property_url;
pub mod vcard_add;

#[derive(Clone)]
pub enum PopupState<'a> {
    Confirm(ConfirmState),
    Export(ExportState<'a>),
    Import(ImportState),
    Message(MessageState),
    PropertyAdr(PropertyAdrState<'a>),
    PropertyBDay(PropertyBDayState<'a>),
    PropertyEmail(PropertyEmailState<'a>),
    PropertyExtra(PropertyExtraState<'a>),
    PropertyN(PropertyNState<'a>),
    PropertyNote(PropertyNoteState<'a>),
    PropertyRemove(PropertyRemoveState),
    PropertyTel(PropertyTelState<'a>),
    PropertyUrl(PropertyUrlState<'a>),
    VcardAdd(VcardAddState<'a>),
}

impl<'a> From<&Property> for PopupState<'a> {
    fn from(property: &Property) -> Self {
        match property.name() {
            PropertyName::ADR => PopupState::PropertyAdr(PropertyAdrState::from(property)),
            PropertyName::BDAY => PopupState::PropertyBDay(PropertyBDayState::from(property)),
            PropertyName::EMAIL => PopupState::PropertyEmail(PropertyEmailState::from(property)),
            PropertyName::N => PopupState::PropertyN(PropertyNState::from(property)),
            PropertyName::NOTE => PopupState::PropertyNote(PropertyNoteState::from(property)),
            PropertyName::TEL => PopupState::PropertyTel(PropertyTelState::from(property)),
            PropertyName::URL => PopupState::PropertyUrl(PropertyUrlState::from(property)),
            _ => PropertyExtra(PropertyExtraState::from(property)),
        }
    }
}

impl<'a> From<&str> for PopupState<'a> {
    fn from(name: &str) -> Self {
        match name {
            PropertyName::ADR => PopupState::PropertyAdr(PropertyAdrState::default()),
            PropertyName::BDAY => PopupState::PropertyBDay(PropertyBDayState::default()),
            PropertyName::EMAIL => PopupState::PropertyEmail(PropertyEmailState::default()),
            PropertyName::N => PopupState::PropertyN(PropertyNState::default()),
            PropertyName::NOTE => PopupState::PropertyNote(PropertyNoteState::default()),
            PropertyName::TEL => PopupState::PropertyTel(PropertyTelState::default()),
            PropertyName::URL => PopupState::PropertyUrl(PropertyUrlState::default()),
            _ => PopupState::Message(MessageState::from(format!("{} is not yet supported.", name))),
        }
    }
}
