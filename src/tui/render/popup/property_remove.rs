use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::state::popup::property_remove::{PropertyRemoveSelected, PropertyRemoveState};
use crate::util::get_block;

pub fn render_property_remove_popup(state: &mut PropertyRemoveState, frame: &mut Frame, rect: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .margin(1)
        .split(rect);
    let row1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(rows[0]);
    let row2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(rows[1]);

    frame.render_widget(Paragraph::new(Text::raw("Are you sure you want to delete this property?")), row1[0]);
    frame.render_widget(
        Paragraph::new(Text::raw("Cancel"))
            .block(get_block(state.selected.is(PropertyRemoveSelected::Cancel), ""))
            .alignment(Alignment::Center),
        row2[0],
    );
    frame.render_widget(
        Paragraph::new(Text::raw("Delete"))
            .block(get_block(state.selected.is(PropertyRemoveSelected::Remove), ""))
            .alignment(Alignment::Center),
        row2[1],
    );
}
