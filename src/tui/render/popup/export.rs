use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::Text;
use tui::widgets::{List, Paragraph};
use tui::Frame;

use crate::state::popup::export::{ExportSelected, ExportState};
use crate::util::{get_block, get_list_item, render_widget_with_textarea};

pub fn render_export_popup(state: &mut ExportState, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
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
        .constraints([Constraint::Percentage(100)])
        .split(rows[1]);
    let row3 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(rows[2]);

    let mut items = Vec::new();
    for (i, (_, filename)) in state.files.files.iter().enumerate() {
        items.push(get_list_item(
            filename.to_string(),
            state.files.list.selected().unwrap_or_default() == i,
        ));
    }

    render_widget_with_textarea(
        "File Name",
        &state.value,
        state.selected.is(ExportSelected::TextArea),
        state.textarea.as_mut(),
        frame,
        row1[0],
    );

    let pathname = state.files.path.file_name().unwrap_or_default().to_str().unwrap_or_default();
    let list = List::new(items).block(get_block(
        state.selected.is(ExportSelected::Files),
        format!("Current Path [{}]", pathname).as_str(),
    ));
    frame.render_stateful_widget(list, row2[0], &mut state.files.list);

    frame.render_widget(
        Paragraph::new(Text::raw("Cancel"))
            .block(get_block(state.selected.is(ExportSelected::Cancel), ""))
            .alignment(Alignment::Center),
        row3[0],
    );
    frame.render_widget(
        Paragraph::new(Text::raw("Export"))
            .block(get_block(state.selected.is(ExportSelected::Export), ""))
            .alignment(Alignment::Center),
        row3[1],
    );
}
