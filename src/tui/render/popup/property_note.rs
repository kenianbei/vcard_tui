use std::borrow::BorrowMut;
use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::Text;
use tui::widgets::Paragraph;
use tui::Frame;

use crate::state::popup::property_note::{PropertyNoteSelected, PropertyNoteState};
use crate::util::{get_block, textarea_with_block};

pub fn render_property_note_popup(state: &mut PropertyNoteState, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
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
            Constraint::Percentage(85),
            Constraint::Percentage(15),
        ])
        .split(rows[1]);

    frame.render_widget(
        textarea_with_block(state.textarea.borrow_mut(), state.selected.is(PropertyNoteSelected::Note), "Notes").widget(),
        row1[0],
    );
    frame.render_widget(
        Paragraph::new(Text::raw("Save"))
            .block(get_block(state.selected.is(PropertyNoteSelected::Save), ""))
            .alignment(Alignment::Center),
        row2[1],
    );
}
