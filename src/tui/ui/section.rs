use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use strum::IntoEnumIterator;

use crate::{
    gamebanana::builder::FeedFilter,
    tui::app::{App, Window},
};

use super::{enum_to_span, hide_unfocused};

pub fn section(frame: &mut Frame, app: &App, area: Rect) {
    let block = hide_unfocused(
        Block::default().title("[4]-Section").borders(Borders::ALL),
        app,
        Window::Section,
    );
    let spans = FeedFilter::iter().map(|f| enum_to_span(f, app.sort.item.clone()));
    let sections = Paragraph::new(Line::from_iter(spans)).block(block);
    frame.render_widget(sections, area);
}
