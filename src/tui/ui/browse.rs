use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List},
    Frame,
};

use crate::tui::app::{App, Window};

pub fn browse_view(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("[2]-Browse")
        .border_style(Style::default().fg(if let Window::Main = app.window.item {
            Color::White
        } else {
            Color::Gray
        }));
    let text = List::new(app.search_items())
        .block(block)
        .highlight_style(Style::default().bg(Color::LightRed));
    frame.render_stateful_widget(text, area, &mut app.search_state().borrow_mut());
}
