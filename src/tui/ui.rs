use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::app::{App, View};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("a", Style::default().fg(Color::Green)))
        .block(title_block.clone());

    let titld =
        Paragraph::new(Text::styled("b", Style::default().fg(Color::Green))).block(title_block);

    match app.view {
        View::Manage => frame.render_widget(&title, chunks[0]),
        View::Browse => frame.render_widget(&titld, chunks[0]),
    }
}
