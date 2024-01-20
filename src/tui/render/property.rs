use ratatui::layout::Rect;
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use vcard_parser::traits::HasName;
use vcard_parser::vcard::property::Property;

use crate::util::{get_block, property_content, property_title};

pub fn render_property(
    property_name: &str,
    property: Option<&Property>,
    selected: bool,
    frame: &mut Frame,
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
