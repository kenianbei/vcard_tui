use std::borrow::BorrowMut;
use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::Text;
use tui::widgets::Paragraph;
use tui::Frame;

use crate::state::popup::vcard_add::{PopupVcardAddSelected, VcardAddState};
use crate::util::{get_block, textarea_with_block};

pub fn render_vcard_add_popup(state: &mut VcardAddState, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
    let rows = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Percentage(100)]).margin(1).split(rect);
    let row1_cols = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(100)]).split(rows[0]);
    let row2_cols = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(85), Constraint::Percentage(15)]).split(rows[1]);

    frame.render_widget(textarea_with_block(state.textarea.borrow_mut(), true, "Full Name").widget(), row1_cols[0]);
    frame.render_widget(Paragraph::new(Text::raw("Create")).block(get_block(state.selected.is(PopupVcardAddSelected::Create), "")).alignment(Alignment::Center), row2_cols[1]);
}
