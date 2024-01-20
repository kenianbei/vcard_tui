use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::state::popup::property_bday::{PropertyBDaySelected, PropertyBDayState};
use crate::util::{get_block, render_widget_with_textarea};

pub fn render_property_bday_popup(state: &mut PropertyBDayState, frame: &mut Frame, rect: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(3),
            Constraint::Length(3),
        ])
        .margin(1)
        .split(rect);
    let row1_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(rows[0]);
    let row2_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(85),
            Constraint::Percentage(15),
        ])
        .split(rows[1]);

    render_widget_with_textarea(
        "Year",
        &state.value_date_data_get().year.to_string(),
        state.selected.is(PropertyBDaySelected::Year),
        state.textarea.as_mut(),
        frame,
        row1_cols[0],
    );
    render_widget_with_textarea(
        "Month",
        &state.value_date_data_get().month.to_string(),
        state.selected.is(PropertyBDaySelected::Month),
        state.textarea.as_mut(),
        frame,
        row1_cols[1],
    );
    render_widget_with_textarea(
        "Day",
        &state.value_date_data_get().day.to_string(),
        state.selected.is(PropertyBDaySelected::Day),
        state.textarea.as_mut(),
        frame,
        row1_cols[2],
    );

    frame.render_widget(
        Paragraph::new(Text::raw("Save"))
            .block(get_block(state.selected.is(PropertyBDaySelected::Save), ""))
            .alignment(Alignment::Center),
        row2_cols[1],
    );
}
