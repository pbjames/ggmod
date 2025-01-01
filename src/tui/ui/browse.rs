use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List},
    Frame,
};

use crate::tui::app::{App, Window};

use super::hide_unfocused;

pub fn browse_view(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = hide_unfocused(
        Block::default().borders(Borders::ALL).title("[2]-Browse"),
        app,
        Window::Main,
    );
    let text = List::new(app.search_items())
        .block(block)
        .highlight_style(Style::default().bg(Color::DarkGray));
    frame.render_stateful_widget(text, area, &mut app.search_state().borrow_mut());
}
