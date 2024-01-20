use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::state::popup::property_extra::{PropertyExtraSelected, PropertyExtraState};
use crate::util::{get_block, render_widget_with_textarea};

const EXTRA_PROPERTIES_HELP: &str = "Custom properties should be formatted as per vCard 4.0 rules.\nFor example: NICKNAME:Johnny";

pub fn render_property_extra_popup(state: &mut PropertyExtraState, frame: &mut Frame, rect: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .margin(1)
        .split(rect);
    let row1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(rows[0]);
    let row2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(rows[1]);
    let row3 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(rows[2]);
    let row4 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(85),
            Constraint::Percentage(15),
        ])
        .split(rows[3]);

    render_widget_with_textarea(
        "Property Name",
        &state.name,
        state.selected.is(PropertyExtraSelected::Name),
        state.textarea.as_mut(),
        frame,
        row1[0],
    );
    render_widget_with_textarea(
        "Property Value",
        &state.value,
        state.selected.is(PropertyExtraSelected::Value),
        state.textarea.as_mut(),
        frame,
        row1[1],
    );
    render_widget_with_textarea(
        "Property Parameters",
        &state.parameters,
        state.selected.is(PropertyExtraSelected::Parameters),
        state.textarea.as_mut(),
        frame,
        row2[0],
    );
    frame.render_widget(Paragraph::new(Text::raw(EXTRA_PROPERTIES_HELP)).alignment(Alignment::Left), row3[0]);
    frame.render_widget(
        Paragraph::new(Text::raw("Save"))
            .block(get_block(state.selected.is(PropertyExtraSelected::Save), ""))
            .alignment(Alignment::Center),
        row4[1],
    );
}
