use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::text::Text;
use tui::widgets::Paragraph;
use tui::Frame;
use vcard_parser::traits::HasName;
use vcard_parser::vcard::property::Property;

use crate::util::{get_block, property_content, property_title};

pub fn render_property(
    property_name: &str,
    property: Option<&Property>,
    selected: bool,
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    rect: Rect,
) {
    if let Some(p) = property {
        frame.render_widget(
            Paragraph::new(Text::raw(property_content(p))).block(get_block(selected, property_title(p.name()))),
            rect,
        );
    } else {
        frame.render_widget(
            Paragraph::new(Text::default()).block(get_block(selected, property_title(&property_name))),
            rect,
        );
    }
}
