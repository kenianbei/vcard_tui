use std::borrow::BorrowMut;
use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;
use vcard_parser::constants::PropertyName;

use crate::state::contact::ContactSelected;
use crate::state::list::StatefulList;
use crate::state::StateSelected;
use crate::tui::render::extra::render_extra_properties_state;
use crate::tui::render::properties::render_properties_state;
use crate::tui::render::property::render_property;
use crate::{State, PRIMARY};

pub fn render_contact(frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect, state: &mut State) {
    frame.render_widget(
        Block::default()
            .title("Contact Information")
            .borders(Borders::ALL)
            .style(Style::default().fg(PRIMARY)),
        rect,
    );

    if let Some(_) = state.contacts.get() {
        let contact_selected = state.selected.is(StateSelected::Contact);

        let row_constraints = [
            Constraint::Length(3),
            Constraint::Max(8),
            Constraint::Max(8),
            Constraint::Max(8),
        ];
        let col_constraints = [
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ];

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .margin(1)
            .split(rect);
        let row1_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(rows[0]);
        let row2_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(rows[1]);
        let row3_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(rows[2]);
        let row4_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(rows[3]);

        render_property(
            PropertyName::N,
            state.contacts.current.property_n.as_ref(),
            contact_selected && state.contacts.current.selected.is(ContactSelected::N),
            frame,
            row1_columns[0],
        );
        render_property(
            PropertyName::BDAY,
            state.contacts.current.property_bday.as_ref(),
            contact_selected && state.contacts.current.selected.is(ContactSelected::BDay),
            frame,
            row1_columns[1],
        );

        render_properties_state(
            state.contacts.current.properties_email.borrow_mut(),
            contact_selected && state.contacts.current.selected.is(ContactSelected::Email),
            frame,
            row2_columns[0],
        );
        render_properties_state(
            state.contacts.current.properties_tel.borrow_mut(),
            contact_selected && state.contacts.current.selected.is(ContactSelected::Tel),
            frame,
            row2_columns[1],
        );
        render_properties_state(
            state.contacts.current.properties_adr.borrow_mut(),
            contact_selected && state.contacts.current.selected.is(ContactSelected::Adr),
            frame,
            row3_columns[0],
        );
        render_properties_state(
            state.contacts.current.properties_url.borrow_mut(),
            contact_selected && state.contacts.current.selected.is(ContactSelected::Url),
            frame,
            row3_columns[1],
        );
        render_properties_state(
            state.contacts.current.properties_note.borrow_mut(),
            contact_selected && state.contacts.current.selected.is(ContactSelected::Note),
            frame,
            row4_columns[0],
        );

        render_extra_properties_state(
            state.contacts.current.properties_extra.borrow_mut(),
            contact_selected && state.contacts.current.selected.is(ContactSelected::Extra),
            frame,
            row4_columns[1],
        );
    } else {
        let area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(48),
                Constraint::Percentage(4),
                Constraint::Percentage(48),
            ])
            .margin(1)
            .split(rect);
        frame.render_widget(
            Paragraph::new(Text::raw("Add [a] or import [i] a contact to get started.")).alignment(Alignment::Center),
            area[1],
        );
    }
}
