use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::Text;
use tui::widgets::Paragraph;
use tui::Frame;

use crate::state::popup::property_adr::{PopupAdrSelected, PropertyAdrState};
use crate::util::{get_block, render_widget_with_textarea};

pub fn render_property_adr_popup(state: &mut PropertyAdrState, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
    let rows = Layout::default().direction(Direction::Vertical).constraints([Constraint::Max(3), Constraint::Max(3), Constraint::Max(3), Constraint::Length(3)]).margin(1).split(rect);
    let row1_cols = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(25), Constraint::Percentage(75)]).split(rows[0]);
    let row2_cols = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(32), Constraint::Percentage(32), Constraint::Percentage(32)]).split(rows[1]);
    let row3_cols = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(100)]).split(rows[2]);
    let row4_cols = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(85), Constraint::Percentage(15)]).split(rows[3]);

    render_widget_with_textarea("Type", &state.values.param_type, state.selected.is(PopupAdrSelected::ParamType), state.textarea.as_mut(), frame, row1_cols[0]);
    render_widget_with_textarea("Street", &state.values.street, state.selected.is(PopupAdrSelected::Street), state.textarea.as_mut(), frame, row1_cols[1]);
    render_widget_with_textarea("City", &state.values.locality, state.selected.is(PopupAdrSelected::Locality), state.textarea.as_mut(), frame, row2_cols[0]);
    render_widget_with_textarea("State", &state.values.region, state.selected.is(PopupAdrSelected::Region), state.textarea.as_mut(), frame, row2_cols[1]);
    render_widget_with_textarea("Zip Code", &state.values.code, state.selected.is(PopupAdrSelected::Code), state.textarea.as_mut(), frame, row2_cols[2]);
    render_widget_with_textarea("Country", &state.values.country, state.selected.is(PopupAdrSelected::Country), state.textarea.as_mut(), frame, row3_cols[0]);

    frame.render_widget(Paragraph::new(Text::raw("Save")).block(get_block(state.selected.is(PopupAdrSelected::Save), "")).alignment(Alignment::Center), row4_cols[1]);
}
