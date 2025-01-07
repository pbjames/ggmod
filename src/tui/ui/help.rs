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
        .border_style(Style::default().fg(Color::LightGreen));
    let text = Paragraph::new(app.help_text()).block(block).centered();
    frame.render_widget(text, area);
}
