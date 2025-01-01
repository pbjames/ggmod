use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, List},
    Frame,
};

use crate::tui::app::{App, Window};

use super::hide_unfocused;

pub fn category(frame: &mut Frame, app: &App, area: Rect) {
    let block = hide_unfocused(
        Block::default().title("[3]-Category").borders(Borders::ALL),
        app,
        Window::Category,
    );
    let iter = app.categories();
    let text = List::new(iter).block(block);
    frame.render_widget(text, area);
}
