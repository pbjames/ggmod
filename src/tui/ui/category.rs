use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::app::{App, Window};

pub fn category(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("[3]-Category")
        .borders(Borders::ALL)
        .border_style(
            Style::default().fg(if let Window::Category = app.window.item {
                Color::White
            } else {
                Color::Gray
            }),
        );
    let text = Paragraph::new("categories").block(block).left_aligned();
    frame.render_widget(text, area);
}
