use crossterm::event::Event;

use crate::state::contact::ContactSelected;
use crate::tui::handler::contact::extra::handle_contact_extra_properties;
use crate::tui::handler::contact::properties::handle_contact_properties;
use crate::tui::handler::contact::property::handle_contact_property;
use crate::State;

mod extra;
mod properties;
mod property;

pub fn handle_contact(event: &Event, state: &mut State) -> anyhow::Result<()> {
    match state.contacts.current.selected.get() {
        ContactSelected::Fn | ContactSelected::BDay => handle_contact_property(event, state)?,
        ContactSelected::Extra => handle_contact_extra_properties(event, state),
        _ => handle_contact_properties(event, state),
    }

    Ok(())
}
