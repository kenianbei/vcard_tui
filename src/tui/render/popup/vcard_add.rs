use ratatui::layout::Rect;
use ratatui::Frame;

use crate::state::popup::vcard_add::VcardAddState;
use crate::tui::render::popup::property_n::render_property_n_popup;

pub fn render_vcard_add_popup(state: &mut VcardAddState, frame: &mut Frame, rect: Rect) {
    render_property_n_popup(&mut state.property_n, frame, rect);
}
