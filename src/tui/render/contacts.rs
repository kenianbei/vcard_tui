use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{List, ListItem};
use tui::Frame;
use vcard_parser::vcard::property::types::PropertyType;

use crate::state::StateSelected;
use crate::util::get_block;
use crate::{State, PRIMARY, PRIMARY_CONTRAST};

pub fn render_contacts(frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect, state: &mut State) {
    let mut items = Vec::new();

    for (i, vcard) in state.contacts.vcards.iter().enumerate() {
        if let Some(property) = vcard.get_property_by_type(&PropertyType::Fn) {
            let mut item = ListItem::new(property.get_value().to_string());
            if let Some(u) = state.contacts.list.selected() {
                if u == i {
                    item = item.style(Style::default().bg(PRIMARY).fg(PRIMARY_CONTRAST));
                }
            }
            items.push(item);
        }
    }

    frame.render_stateful_widget(List::new(items).block(get_block(state.selected.is(StateSelected::Contacts), "Contacts")), rect, &mut state.contacts.list);
}
