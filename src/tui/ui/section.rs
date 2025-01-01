use crate::tui::app::{App, Window};
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::{enum_to_span, hide_unfocused};

pub fn section(frame: &mut Frame, app: &App, area: Rect) {
    let block = hide_unfocused(
        Block::default().title("[4]-Section").borders(Borders::ALL),
        app,
        Window::Section,
    );
    let sections =
        Paragraph::new(Line::from_iter(enum_to_span(app.section.item.clone()))).block(block);
    frame.render_widget(sections, area);
}
