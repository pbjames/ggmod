use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use crate::tui::app::{App, Window};

use super::hide_unfocused;

pub fn browse_view(frame: &mut Frame, app: &mut App, area: Rect) {
    let widths = [
        Constraint::Length(35),
        Constraint::Length(20),
        Constraint::Max(6),
        Constraint::Max(6),
        Constraint::Max(10),
        Constraint::Fill(1),
    ];
    let block = hide_unfocused(
        Block::default().borders(Borders::ALL).title("[2]-Browse"),
        app,
        Window::Main,
    );
    let header = Row::new(vec![
        "Name",
        "Character",
        "Views",
        "Likes",
        "Downloads",
        "Description",
    ]);
    let text = Table::new(app.online_items_repr(), widths)
        .header(header)
        .widths(widths)
        .block(block)
        .row_highlight_style(Style::default().bg(Color::DarkGray));
    frame.render_stateful_widget(text, area, &mut app.search_state().borrow_mut());
}
