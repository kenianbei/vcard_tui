use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::{List, Paragraph};
use ratatui::Frame;

use crate::state::popup::import::{ImportSelected, ImportState};
use crate::util::{get_block, get_list_item};

pub fn render_import_popup(state: &mut ImportState, frame: &mut Frame, rect: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),
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
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(rows[1]);

    let mut items = Vec::new();
    for (i, (_, filename)) in state.files.files.iter().enumerate() {
        items.push(get_list_item(
            filename.to_string(),
            state.files.list.selected().unwrap_or_default() == i,
        ));
    }

    frame.render_stateful_widget(
        List::new(items).block(get_block(state.selected.is(ImportSelected::Files), "Files")),
        row1[0],
        &mut state.files.list,
    );
    frame.render_widget(
        Paragraph::new(Text::raw("Cancel"))
            .block(get_block(state.selected.is(ImportSelected::Cancel), ""))
            .alignment(Alignment::Center),
        row2[0],
    );
    frame.render_widget(
        Paragraph::new(Text::raw("Import"))
            .block(get_block(state.selected.is(ImportSelected::Import), ""))
            .alignment(Alignment::Center),
        row2[1],
    );
}
