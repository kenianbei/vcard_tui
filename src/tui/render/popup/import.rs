use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{List, ListItem, Paragraph};
use tui::Frame;

use crate::state::popup::import::{ImportState, PopupImportSelected};
use crate::util::get_block;

pub fn render_import_popup(state: &mut ImportState, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
    let rows = Layout::default().direction(Direction::Vertical).constraints([Constraint::Min(10), Constraint::Length(3)]).margin(1).split(rect);
    let row1 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(100)]).split(rows[0]);
    let row2 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[1]);

    let mut items = Vec::new();
    for (i, (_, filename)) in state.files.files.iter().enumerate() {
        let mut item = ListItem::new(filename.clone());
        if let Some(u) = state.files.list.selected() {
            if u == i {
                item = item.style(Style::default().bg(Color::White).fg(Color::Black));
            }
        }
        items.push(item);
    }

    frame.render_stateful_widget(List::new(items).block(get_block(state.selected.is(PopupImportSelected::Files), "Files")), row1[0], &mut state.files.list);
    frame.render_widget(Paragraph::new(Text::raw("Cancel")).block(get_block(state.selected.is(PopupImportSelected::Cancel), "")).alignment(Alignment::Center), row2[0]);
    frame.render_widget(Paragraph::new(Text::raw("Import")).block(get_block(state.selected.is(PopupImportSelected::Import), "")).alignment(Alignment::Center), row2[1]);
}
