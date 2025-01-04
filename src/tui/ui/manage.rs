use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List},
    Frame,
};

use crate::tui::{
    app::{App, View, ViewDir, Window},
    state::ItemizedState,
};

pub fn manage_view(frame: &mut Frame, app: &App, area: Rect) {
    let halves = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    let block_left = Block::default()
        .borders(Borders::ALL)
        .title("[2]-Staged")
        .style(Style::default().fg(match app.view {
            View::Manage(ViewDir::Left) if app.window.item == Window::Main => Color::White,
            _ => Color::DarkGray,
        }));
    let block_right = Block::default()
        .borders(Borders::ALL)
        .title("Unstaged")
        .style(Style::default().fg(match app.view {
            View::Manage(ViewDir::Right) if app.window.item == Window::Main => Color::White,
            _ => Color::DarkGray,
        }));
    let left = List::new(app.staged_items.items()).block(block_left);
    let right = List::new(app.unstaged_items.items()).block(block_right);
    //frame.render_widget(block, area);
    frame.render_widget(left, halves[0]);
    frame.render_widget(right, halves[1]);
}
