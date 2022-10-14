use vcard_parser::vcard::property::types::PropertyType;
use vcard_parser::vcard::property::Property;

use crate::state::popup::confirm::ConfirmState;
use crate::state::popup::export::ExportState;
use crate::state::popup::import::ImportState;
use crate::state::popup::message::MessageState;
use crate::state::popup::property_adr::PropertyAdrState;
use crate::state::popup::property_email::PropertyEmailState;
use crate::state::popup::property_extra::PropertyExtraState;
use crate::state::popup::property_note::PropertyNoteState;
use crate::state::popup::property_remove::PropertyRemoveState;
use crate::state::popup::property_tel::PropertyTelState;
use crate::state::popup::property_url::PropertyUrlState;
use crate::state::popup::vcard_add::VcardAddState;

pub mod confirm;
pub mod export;
pub mod import;
pub mod message;
pub mod property_adr;
pub mod property_email;
pub mod property_extra;
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
    PropertyEmail(PropertyEmailState<'a>),
    PropertyExtra(PropertyExtraState<'a>),
    PropertyNote(PropertyNoteState<'a>),
    PropertyRemove(PropertyRemoveState),
    PropertyTel(PropertyTelState<'a>),
    PropertyUrl(PropertyUrlState<'a>),
    VcardAdd(VcardAddState<'a>),
}

impl<'a> From<&Property> for PopupState<'a> {
    fn from(property: &Property) -> Self {
        match property.get_type() {
            PropertyType::Adr => PopupState::PropertyAdr(PropertyAdrState::from(property)),
            PropertyType::Email => PopupState::PropertyEmail(PropertyEmailState::from(property)),
            PropertyType::Note => PopupState::PropertyNote(PropertyNoteState::from(property)),
            PropertyType::Tel => PopupState::PropertyTel(PropertyTelState::from(property)),
            PropertyType::Url => PopupState::PropertyUrl(PropertyUrlState::from(property)),
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<&PropertyType> for PopupState<'a> {
    fn from(property_type: &PropertyType) -> Self {
        match property_type {
            PropertyType::Adr => PopupState::PropertyAdr(PropertyAdrState::default()),
            PropertyType::Email => PopupState::PropertyEmail(PropertyEmailState::default()),
            PropertyType::Note => PopupState::PropertyNote(PropertyNoteState::default()),
            PropertyType::Tel => PopupState::PropertyTel(PropertyTelState::default()),
            PropertyType::Url => PopupState::PropertyUrl(PropertyUrlState::default()),
            _ => unimplemented!(),
        }
    }
}
