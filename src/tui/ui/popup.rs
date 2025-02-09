use ratatui::{
    layout::{Constraint, Rect},
    style::Color,
    widgets::{Block, Borders, Clear, Row, Table},
    Frame,
};

use crate::tui::{app::App, state::Itemized};

pub fn try_popup(frame: &mut Frame, app: &mut App, area: Rect) {
    if app.popup_items.is_empty() {
        return;
    }
    let header = Row::new(vec!["Name", "Downloads", "Description"]);
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Select a variant");
    let widths = [
        Constraint::Length(20),
        Constraint::Length(9),
        Constraint::Fill(1),
    ];
    let table = Table::new(app.popup_items.content.clone(), widths)
        .widths(widths)
        .block(block)
        .header(header)
        .row_highlight_style(Color::Yellow);
    frame.render_widget(Clear, area);
    frame.render_stateful_widget(table, area, &mut app.popup_items.state.borrow_mut());
}
