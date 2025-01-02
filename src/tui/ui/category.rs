use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List},
    Frame,
};

use crate::tui::app::{App, Window};

use super::hide_unfocused;

pub fn category(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = hide_unfocused(
        Block::default().title("[3]-Category").borders(Borders::ALL),
        app,
        Window::Category,
    );
    let iter = app.categories();
    let text = List::new(iter)
        .block(block)
        .highlight_style(Style::default().fg(Color::Blue));
    frame.render_stateful_widget(text, area, &mut app.categories.state.borrow_mut());
}
