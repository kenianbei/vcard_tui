use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::state::popup::message::MessageState;
use crate::util::get_block;

pub fn render_message_popup(state: &mut MessageState, frame: &mut Frame, rect: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Max(3),
        ])
        .margin(1)
        .horizontal_margin(2)
        .split(rect);
    let row1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(rows[0]);
    let row2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(75),
            Constraint::Percentage(25),
        ])
        .split(rows[1]);

    frame.render_widget(Paragraph::new(Text::raw(state.value.as_str())), row1[0]);
    frame.render_widget(
        Paragraph::new(Text::raw("Ok"))
            .block(get_block(true, ""))
            .alignment(Alignment::Center),
        row2[1],
    );
}
