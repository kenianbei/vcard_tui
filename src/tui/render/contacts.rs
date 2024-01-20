use ratatui::layout::Rect;
use ratatui::widgets::List;
use ratatui::Frame;
use vcard_parser::constants::PropertyName;
use vcard_parser::vcard::property::property_fn::PropertyFnData;
use vcard_parser::vcard::property::Property::PropertyFn;

use crate::state::StateSelected;
use crate::util::{get_block, get_list_item, property_content};
use crate::State;

pub fn render_contacts(frame: &mut Frame, rect: Rect, state: &mut State) {
    let mut items = Vec::new();

    for (i, vcard) in state.contacts.vcards.iter().enumerate() {
        items.push(get_list_item(
            property_content(
                &vcard
                    .get_property_by_name(PropertyName::FN)
                    .unwrap_or(PropertyFn(PropertyFnData::from("FN:No Name\n"))),
            ),
            state.contacts.list.selected().unwrap_or_default() == i,
        ));
    }

    frame.render_stateful_widget(
        List::new(items).block(get_block(state.selected.is(StateSelected::Contacts), "Contacts")),
        rect,
        &mut state.contacts.list,
    );
}
