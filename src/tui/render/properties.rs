use std::borrow::BorrowMut;

use ratatui::layout::Rect;
use ratatui::widgets::List;
use ratatui::Frame;

use crate::state::properties::PropertiesState;
use crate::util::{get_block, get_list_item, property_content, property_title};

pub fn render_properties_state(state: &mut PropertiesState, selected: bool, frame: &mut Frame, rect: Rect) {
    let mut items = Vec::new();

    for (i, p) in state.properties.iter().enumerate() {
        items.push(get_list_item(
            property_content(p),
            selected && state.list.selected().unwrap_or_default() == i,
        ));
    }

    frame.render_stateful_widget(
        List::new(items).block(get_block(selected, property_title(&state.property_name))),
        rect,
        state.list.borrow_mut(),
    );
}
