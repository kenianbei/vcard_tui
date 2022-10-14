use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{List, ListItem, Paragraph};
use tui::Frame;

use crate::state::popup::export::{ExportState, PopupExportSelected};
use crate::util::{get_block, render_widget_with_textarea};

pub fn render_export_popup(state: &mut ExportState, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
    let rows = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)]).margin(1).split(rect);
    let row1 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(100)]).split(rows[0]);
    let row2 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(100)]).split(rows[1]);
    let row3 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[2]);

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

    render_widget_with_textarea("File Name", &state.value, state.selected.is(PopupExportSelected::TextArea), state.textarea.as_mut(), frame, row1[0]);

    let pathname = state.files.path.file_name().unwrap_or_default().to_str().unwrap_or_default();
    let list = List::new(items).block(get_block(state.selected.is(PopupExportSelected::Files), format!("Current Path [{}]", pathname).as_str()));
    frame.render_stateful_widget(list, row2[0], &mut state.files.list);

    frame.render_widget(Paragraph::new(Text::raw("Cancel")).block(get_block(state.selected.is(PopupExportSelected::Cancel), "")).alignment(Alignment::Center), row3[0]);
    frame.render_widget(Paragraph::new(Text::raw("Export")).block(get_block(state.selected.is(PopupExportSelected::Export), "")).alignment(Alignment::Center), row3[1]);
}
