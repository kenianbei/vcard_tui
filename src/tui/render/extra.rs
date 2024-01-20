use std::borrow::BorrowMut;

use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Text;
use ratatui::widgets::{List, ListItem};
use ratatui::Frame;

use crate::state::extra::ExtraPropertiesState;
use crate::util::get_block;
use crate::{PRIMARY, PRIMARY_CONTRAST};

pub fn render_extra_properties_state(
    extra: &mut ExtraPropertiesState,
    selected: bool,
    frame: &mut Frame,
    rect: Rect,
) {
    let mut items = Vec::new();

    for (i, p) in extra.properties.iter().enumerate() {
        let mut item = ListItem::new(Text::raw(p.export()));
        if let Some(u) = extra.list.selected() {
            if u == i {
                item = item.style(Style::default().bg(PRIMARY).fg(PRIMARY_CONTRAST));
            }
        }
        items.push(item);
    }

    frame.render_stateful_widget(
        List::new(items).block(get_block(selected, "Extra Properties")),
        rect,
        extra.list.borrow_mut(),
    );
}
