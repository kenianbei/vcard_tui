use std::io;
use std::io::Stdout;

use crossterm::event::{DisableMouseCapture, Event, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;
use tui_textarea::{CursorMove, TextArea};
use vcard_parser::traits::HasName;
use vcard_parser::vcard::parameter::parameter_type::ParameterTypeData;
use vcard_parser::vcard::parameter::Parameter;
use vcard_parser::vcard::parameter::Parameter::ParameterType;
use vcard_parser::vcard::value::value_listcomponent::ValueListComponentData;

use crate::state::contacts::ContactsState;
use crate::state::StateSelected;
use crate::tui::handler::contact::handle_contact;
use crate::tui::handler::contacts::handle_contacts;
use crate::tui::handler::handle;
use crate::tui::handler::popup::handle_popup;
use crate::tui::render::contact::render_contact;
use crate::tui::render::contacts::render_contacts;
use crate::tui::render::popup::render_popup;
use crate::tui::render::toolbar::render_toolbar;
use crate::State;

mod handler;
mod render;

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl From<Terminal<CrosstermBackend<Stdout>>> for Tui {
    fn from(terminal: Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self { terminal }
    }
}

impl Tui {
    pub fn create() -> anyhow::Result<Self> {
        Ok(Tui::from(Terminal::new(CrosstermBackend::new(io::stdout()))?))
    }
    pub fn start(&mut self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        io::stdout().execute(EnterAlternateScreen)?;
        io::stdout().execute(DisableMouseCapture)?;
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }
    pub fn render(&mut self, state: &mut State) -> anyhow::Result<()> {
        self.terminal.draw(|frame| {
            let rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(95),
                    Constraint::Length(3),
                ])
                .split(frame.size());
            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(75),
                ])
                .split(rows[0]);

            render_contacts(frame, cols[0], state);
            render_contact(frame, cols[1], state);
            render_toolbar(frame, rows[1], state);
            if let Some(popup) = state.popup.as_mut() {
                render_popup(frame, cols[1], popup)
            }
        })?;
        Ok(())
    }
    pub fn input(&self, event: &Event, state: &mut State) -> anyhow::Result<()> {
        if state.popup.is_some() {
            handle_popup(event, state)?;
        } else {
            handle(event, state)?;
            match state.selected.get() {
                StateSelected::Contacts => handle_contacts(event, state)?,
                StateSelected::Contact => handle_contact(event, state)?,
            }
        }
        Ok(())
    }
    pub fn end(&mut self) -> anyhow::Result<()> {
        self.terminal.clear()?;
        self.terminal.show_cursor()?;
        io::stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}

pub trait KeyEventHandler {
    fn handle_key_event(&mut self, key_event: &KeyEvent, state: &mut ContactsState) -> anyhow::Result<bool>;
}

pub trait HasValueListComponent {
    fn value_data_get(&self) -> ValueListComponentData;
    fn value_data_set(&mut self, data: ValueListComponentData);
    fn value_listcomponent_get_string(&self, i: usize) -> String {
        if let Some(v) = self.value_data_get().value.get(i) {
            if let Some(v) = v.get(0) {
                return v.to_string();
            }
        }
        String::new()
    }
    fn value_listcomponent_set_string(&mut self, i: usize, string: String) {
        let mut data = self.value_data_get().clone();
        data.value[i] = Vec::from([string]);
        self.value_data_set(data);
    }
}

pub trait HasTextArea<'a> {
    fn textarea_value_get(&self) -> String;
    fn textarea_value_set(&mut self, string: String);
    fn textarea_get(&self) -> &Option<TextArea>;
    fn textarea_set(&mut self, textarea: Option<TextArea<'a>>);
    fn toggle_textarea(&mut self) {
        if let Some(textarea) = self.textarea_get() {
            self.textarea_value_set(textarea.lines().first().unwrap().to_string());
            self.textarea_set(None)
        } else {
            let mut textarea = TextArea::new(Vec::from([self.textarea_value_get()]));
            textarea.move_cursor(CursorMove::End);
            self.textarea_set(Some(textarea))
        }
    }
}

pub trait HasTypeParam {
    fn parameters_get(&self) -> Vec<Parameter>;
    fn parameters_set(&mut self, parameters: Vec<Parameter>);
    fn type_param_get(&self) -> Parameter {
        self.parameters_get()
            .iter()
            .find(|p| p.name() == "TYPE")
            .cloned()
            .unwrap_or(ParameterType(ParameterTypeData::default()))
    }
    fn type_param_set(&mut self, string: String) {
        let mut parameters = self
            .parameters_get()
            .into_iter()
            .filter(|p| p.name() != "TYPE")
            .collect::<Vec<Parameter>>();
        parameters.push(ParameterType(
            ParameterTypeData::try_from(string.as_str()).unwrap_or(ParameterTypeData::default()),
        ));
        self.parameters_set(parameters);
    }
}
