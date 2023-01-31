use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::Frame;

use crate::state::popup::vcard_add::VcardAddState;
use crate::tui::render::popup::property_n::render_property_n_popup;

pub fn render_vcard_add_popup(state: &mut VcardAddState, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect) {
    render_property_n_popup(&mut state.property_n, frame, rect);
}
