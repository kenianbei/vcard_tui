use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::Text;
use tui::widgets::Paragraph;
use tui::Frame;

use crate::state::popup::property_extra::{PopupExtraSelected, PropertyExtraState};
use crate::util::{get_block, render_widget_with_textarea};

const EXTRA_PROPERTIES_HELP: &str = r"
Custom properties should be formatted
as per vCard 4.0 rules. For example: 
NICKNAME:Johnny
";

pub fn render_property_extra_popup(state: &mut PropertyExtraState, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
    let rows = Layout::default().direction(Direction::Vertical).constraints([Constraint::Max(3), Constraint::Min(5), Constraint::Length(3)]).margin(1).split(rect);
    let row1 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(100)]).split(rows[0]);
    let row2 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(100)]).split(rows[1]);
    let row3 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(85), Constraint::Percentage(15)]).split(rows[2]);

    render_widget_with_textarea("Custom Property", &state.value, state.selected.is(PopupExtraSelected::Extra), state.textarea.as_mut(), frame, row1[0]);
    frame.render_widget(Paragraph::new(Text::raw(EXTRA_PROPERTIES_HELP)).alignment(Alignment::Left), row2[0]);
    frame.render_widget(Paragraph::new(Text::raw("Save")).block(get_block(state.selected.is(PopupExtraSelected::Save), "")).alignment(Alignment::Center), row3[1]);
}
