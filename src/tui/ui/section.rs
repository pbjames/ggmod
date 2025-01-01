use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    gamebanana::builder::TypeFilter,
    tui::app::{App, Window},
};

pub fn section(frame: &mut Frame, app: &App, area: Rect) {
    // TODO: This style of code is straight ass
    let block = Block::default()
        .title("[4]-Section")
        .borders(Borders::ALL)
        .border_style(
            Style::default().fg(if let Window::Section = app.window.item {
                Color::White
            } else {
                Color::Gray
            }),
        );
    let sections = Paragraph::new(vec![
        Line::from(Span::styled(
            "Mod",
            Style::default().bg(if let TypeFilter::Mod = app.section.item {
                Color::DarkGray
            } else {
                Color::Black
            }),
        )),
        Line::from(Span::styled(
            "Sound",
            Style::default().bg(if let TypeFilter::Sound = app.section.item {
                Color::DarkGray
            } else {
                Color::Black
            }),
        )),
        Line::from(Span::styled(
            "WiP",
            Style::default().bg(if let TypeFilter::WiP = app.section.item {
                Color::DarkGray
            } else {
                Color::Black
            }),
        )),
    ])
    .block(block);
    frame.render_widget(sections, area);
}
