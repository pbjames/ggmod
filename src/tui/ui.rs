use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{self, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::app::{App, View};

pub fn ui(frame: &mut Frame, app: &App) {
    let view_and_cat = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Percentage(25)])
        .split(frame.area());
    let view_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(view_and_cat[0]);

    search_bar(frame, app, view_chunks[0]);
    match app.view {
        View::Manage => manage_view(frame, view_chunks[1]),
        View::Browse => browse_view(frame, app, view_chunks[1]),
    }
}

pub fn search_bar(frame: &mut Frame, app: &App, area: Rect) {
    let text_block = Block::default().borders(Borders::ALL).title("[1]-Search");
    let text = Paragraph::new(app.search.clone())
        .block(text_block)
        .left_aligned();
    frame.render_widget(text, area);
}

pub fn manage_view(frame: &mut Frame, area: Rect) {
    let halves = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    let text_block = Block::default()
        .borders(Borders::ALL)
        .title("[3]-Manage-Mods");
    let text_left = Paragraph::new("left")
        .block(text_block.clone())
        .left_aligned();
    let text_right = Paragraph::new("right")
        .block(text_block.clone())
        .left_aligned();
    frame.render_widget(text_block, area);
    frame.render_widget(text_left, halves[0]);
    frame.render_widget(text_right, halves[1]);
}

pub fn browse_view(frame: &mut Frame, app: &App, area: Rect) {
    let text_block = Block::default().borders(Borders::ALL).title("[3]-Browse");
    let text = Paragraph::new("Browse internet")
        .block(text_block)
        .left_aligned();
    frame.render_widget(text, area);
}
