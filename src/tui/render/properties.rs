use std::borrow::BorrowMut;
use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::style::Style;
use tui::text::Text;
use tui::widgets::{List, ListItem};
use tui::Frame;

use crate::state::contact::properties::PropertiesState;
use crate::util::{get_block, property_title};
use crate::{PRIMARY, PRIMARY_CONTRAST};

pub fn render_properties_state(state: &mut PropertiesState, selected: bool, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
    let mut items = Vec::new();

    for (i, p) in state.properties.iter().enumerate() {
        let mut item = ListItem::new(Text::raw(state.get_content(p)));
        if let Some(u) = state.list.selected() {
            if u == i && selected {
                item = item.style(Style::default().bg(PRIMARY).fg(PRIMARY_CONTRAST));
            }
        }
        items.push(item);
    }

    frame.render_stateful_widget(List::new(items).block(get_block(selected, property_title(&state.property_type))), rect, state.list.borrow_mut());
}
