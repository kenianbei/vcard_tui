use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::state::popup::property_n::{PropertyNSelected, PropertyNState};
use crate::tui::HasValueListComponent;
use crate::util::{get_block, render_widget_with_textarea};

pub fn render_property_n_popup(state: &mut PropertyNState, frame: &mut Frame, rect: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .margin(1)
        .split(rect);
    let row1_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
            Constraint::Percentage(30),
        ])
        .split(rows[0]);
    let row2_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(rows[1]);
    let row3_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(85),
            Constraint::Percentage(15),
        ])
        .split(rows[2]);

    render_widget_with_textarea(
        "First Name",
        &state.value_listcomponent_get_string(1),
        state.selected.is(PropertyNSelected::Given),
        state.textarea.as_mut(),
        frame,
        row1_cols[0],
    );
    render_widget_with_textarea(
        "Family Name",
        &state.value_listcomponent_get_string(0),
        state.selected.is(PropertyNSelected::Family),
        state.textarea.as_mut(),
        frame,
        row1_cols[1],
    );
    render_widget_with_textarea(
        "Middle Name(s)",
        &state.value_listcomponent_get_string(2),
        state.selected.is(PropertyNSelected::Additional),
        state.textarea.as_mut(),
        frame,
        row2_cols[0],
    );

    frame.render_widget(
        Paragraph::new(Text::raw("Save"))
            .block(get_block(state.selected.is(PropertyNSelected::Save), ""))
            .alignment(Alignment::Center),
        row3_cols[1],
    );
}
