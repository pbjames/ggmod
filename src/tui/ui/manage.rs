use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List},
    Frame,
};

use crate::tui::app::{App, Window};

pub fn manage_view(frame: &mut Frame, app: &App, area: Rect) {
    let halves = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    let block = Block::default()
        .borders(Borders::ALL)
        .title("[2]-Manage-Mods")
        .border_style(Style::default().fg(if let Window::Main = app.window.item {
            Color::White
        } else {
            Color::Gray
        }));
    let left = List::new(app.staged()).block(block.clone());
    let right = List::new(app.unstaged()).block(block.clone());
    frame.render_widget(block, area);
    frame.render_widget(left, halves[0]);
    frame.render_widget(right, halves[1]);
}
