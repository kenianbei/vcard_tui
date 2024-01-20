use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::state::contact::ContactSelected;
use crate::state::StateSelected;
use crate::{State, PRIMARY};

pub fn render_toolbar(frame: &mut Frame, rect: Rect, state: &mut State) {
    let mut text = String::from(" Quit [q] | Import Contacts [i] | Export Contacts [e] ");
    match state.selected.get() {
        StateSelected::Contacts => text.push_str("| Add Contact [a] | Delete Contact [d] | Sort Contacts [s] "),
        StateSelected::Contact => {
            let single = "| Edit [⏎] ";
            let multiple = "| Add [a] | Delete Property [d] | Edit [⏎] ";
            match state.contacts.current.selected.get() {
                ContactSelected::N => text.push_str(single),
                ContactSelected::BDay => text.push_str(single),
                ContactSelected::Email => text.push_str(multiple),
                ContactSelected::Tel => text.push_str(multiple),
                ContactSelected::Adr => text.push_str(multiple),
                ContactSelected::Url => text.push_str(multiple),
                ContactSelected::Note => text.push_str(multiple),
                ContactSelected::Extra => text.push_str(multiple),
            }
        }
    };
    frame.render_widget(
        Paragraph::new(Text::raw(text)).block(
            Block::default()
                .title("Commands")
                .borders(Borders::ALL)
                .style(Style::default().fg(PRIMARY)),
        ),
        rect,
    );
}
