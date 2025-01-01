use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::app::App;

pub fn help_window(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("[?]-Help")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));
    let text = Paragraph::new(String::from("\n") + app.help_text())
        .block(block)
        .centered();
    frame.render_widget(text, area);
}
