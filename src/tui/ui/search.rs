use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
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

pub fn search_bar(frame: &mut Frame, app: &App, area: Rect) {
    let search_and_sort = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(30)])
        .split(area);
    let block = hide_unfocused(
        Block::default().title("[1]-Search").borders(Borders::ALL),
        app,
        Window::Search,
    );
    let search = Paragraph::new(app.search_query().clone())
        .block(block.clone())
        .left_aligned();
    let sorts = sorts_attachment_widget(app);
    frame.render_widget(search, search_and_sort[0]);
    frame.render_widget(sorts, search_and_sort[1]);
}

fn sorts_attachment_widget<'a>(app: &App) -> Paragraph<'a> {
    let block = hide_unfocused(
        Block::default()
            .title("Sort")
            .borders(Borders::LEFT.complement()),
        app,
        Window::Search,
    );
    let spans = FeedFilter::iter().map(|f| enum_to_span(f, app.sort.item.clone()));
    Paragraph::new(Line::from_iter(spans).centered()).block(block)
}
