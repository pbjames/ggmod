use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    gamebanana::builder::FeedFilter,
    tui::app::{App, Window},
};

pub fn search_bar(frame: &mut Frame, app: &App, area: Rect) {
    let search_and_sort = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(30)])
        .split(area);
    let block = Block::default()
        .title("[1]-Search")
        .borders(Borders::ALL)
        .border_style(
            Style::default().fg(if let Window::Search = app.window.item {
                Color::White
            } else {
                Color::Gray
            }),
        );
    let block2 = Block::default()
        .title("Sort")
        .borders(Borders::LEFT.complement())
        .border_style(
            Style::default().fg(if let Window::Search = app.window.item {
                Color::White
            } else {
                Color::Gray
            }),
        );
    let search = Paragraph::new(app.search_query().clone())
        .block(block.clone())
        .left_aligned();
    let sorts = Paragraph::new(
        Line::from_iter([
            Span::styled(
                "Recent",
                Style::default().bg(if let FeedFilter::Recent = app.sort.item {
                    Color::DarkGray
                } else {
                    Color::Black
                }),
            ),
            Span::raw("\n"),
            Span::styled(
                "Popular",
                Style::default().bg(if let FeedFilter::Popular = app.sort.item {
                    Color::DarkGray
                } else {
                    Color::Black
                }),
            ),
            Span::raw("\n"),
            Span::styled(
                "Featured",
                Style::default().bg(if let FeedFilter::Featured = app.sort.item {
                    Color::DarkGray
                } else {
                    Color::Black
                }),
            ),
        ])
        .centered(),
    )
    .block(block2);
    frame.render_widget(search, search_and_sort[0]);
    frame.render_widget(sorts, search_and_sort[1]);
}
