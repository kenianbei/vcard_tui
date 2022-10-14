use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::Frame;

use crate::state::contact::property::PropertyState;
use crate::util::{property_title, render_widget_with_textarea};

pub fn render_property(state: &mut PropertyState, selected: bool, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
    render_widget_with_textarea(property_title(&state.property_type), state.value.as_str(), selected, state.textarea.as_mut(), frame, rect);
}
