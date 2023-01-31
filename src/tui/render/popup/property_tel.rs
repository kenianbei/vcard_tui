use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::Text;
use tui::widgets::Paragraph;
use tui::Frame;
use vcard_parser::traits::HasValue;

use crate::state::popup::property_tel::{PropertyTelSelected, PropertyTelState};
use crate::tui::HasTypeParam;
use crate::util::{get_block, render_widget_with_textarea};

pub fn render_property_tel_popup(state: &mut PropertyTelState, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
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
            Constraint::Percentage(25),
            Constraint::Percentage(75),
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
        "Type",
        &state.type_param_get().get_value().to_string(),
        state.selected.is(PropertyTelSelected::ParamType),
        state.textarea.as_mut(),
        frame,
        row1_cols[0],
    );
    render_widget_with_textarea(
        "Tel",
        &state.property.get_value().to_string(),
        state.selected.is(PropertyTelSelected::Tel),
        state.textarea.as_mut(),
        frame,
        row1_cols[1],
    );

    frame.render_widget(
        Paragraph::new(Text::raw("Save"))
            .block(get_block(state.selected.is(PropertyTelSelected::Save), ""))
            .alignment(Alignment::Center),
        row2_cols[1],
    );
}
