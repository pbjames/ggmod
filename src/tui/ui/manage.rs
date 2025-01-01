use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List},
    Frame,
};

use crate::tui::app::{App, Window};

use super::hide_unfocused;

pub fn manage_view(frame: &mut Frame, app: &App, area: Rect) {
    let halves = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    let block = hide_unfocused(
        Block::default()
            .borders(Borders::ALL)
            .title("[2]-Manage-Mods"),
        app,
        Window::Main,
    );
    let left = List::new(app.staged()).block(block.clone());
    let right = List::new(app.unstaged()).block(block.clone());
    frame.render_widget(block, area);
    frame.render_widget(left, halves[0]);
    frame.render_widget(right, halves[1]);
}
