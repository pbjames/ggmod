use crate::tui::app::{App, Window};
use ratatui::widgets::List;
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders},
    Frame,
};

use super::{enum_to_span, hide_unfocused};

pub fn section(frame: &mut Frame, app: &App, area: Rect) {
    let block = hide_unfocused(
        Block::default().title("[4]-Section").borders(Borders::ALL),
        app,
        Window::Section,
    );
    let spans = enum_to_span(app.section.item.clone());
    let sections = List::new(spans).block(block);
    frame.render_widget(sections, area);
}
