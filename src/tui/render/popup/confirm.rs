use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::Text;
use tui::widgets::Paragraph;
use tui::Frame;

use crate::state::popup::confirm::{ConfirmKind, ConfirmSelected, ConfirmState};
use crate::util::get_block;

pub fn render_confirm_popup(state: &mut ConfirmState, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .margin(1)
        .horizontal_margin(3)
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

    let text = match state.kind {
        ConfirmKind::Quit => "Are you sure you want to quit?",
        ConfirmKind::Delete => "Are you sure you want to delete this vCard?",
    };
    frame.render_widget(Paragraph::new(Text::raw(text)), row1[0]);
    frame.render_widget(
        Paragraph::new(Text::raw("Cancel"))
            .block(get_block(state.selected.is(ConfirmSelected::Cancel), ""))
            .alignment(Alignment::Center),
        row2[0],
    );
    frame.render_widget(
        Paragraph::new(Text::raw("Confirm"))
            .block(get_block(state.selected.is(ConfirmSelected::Confirm), ""))
            .alignment(Alignment::Center),
        row2[1],
    );
}
