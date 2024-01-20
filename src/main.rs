//! # vCard TUI
//!
//! Terminal UI application for managing vCard contacts, written in Rust. It uses the vCard format for importing and exporting contact data.
//!
//! ## Install
//!
//! ```shell
//! cargo install vcard_tui
//! ```
//!
//! ## Usage
//!
//! ```shell
//! vct
//! ```

use std::borrow::BorrowMut;

use ratatui::style::Color;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};

use crate::state::popup::message::MessageState;
use crate::state::popup::PopupState;
use crate::state::State;
use crate::tui::Tui;

mod state;
mod tui;
mod util;

const PRIMARY: Color = Color::Rgb(223, 213, 165);
const PRIMARY_CONTRAST: Color = Color::Black;
const PRIMARY_FOCUSED: Color = Color::Rgb(203, 145, 82);

fn main() -> anyhow::Result<()> {
    let mut state = State::default();
    let mut tui = Tui::create()?;

    tui.start()?;

    loop {
        if let Err(error) = tui.render(state.borrow_mut()) {
            state.popup = Some(PopupState::Message(MessageState::from(error.to_string())))
        }

        let event = read()?;
        if let Event::Key(key_event) = event {
            if key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                state.render = false;
            }
        }

        if let Err(error) = tui.input(&event, state.borrow_mut()) {
            state.popup = Some(PopupState::Message(MessageState::from(error.to_string())))
        }

        if !state.render {
            break;
        }
    }

    tui.end()?;

    Ok(())
}
