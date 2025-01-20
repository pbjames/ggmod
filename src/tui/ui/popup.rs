use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::{Block, Borders, Clear, Row, Table},
    Frame,
};

use crate::tui::app::App;

pub fn popup(frame: &mut Frame, app: &mut App, area: Rect) {
    let header = Row::new(vec!["Name", "Downloads", "Description"]);
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Select a variant");
    let widths = [
        Constraint::Length(35),
        Constraint::Length(9),
        Constraint::Fill(1),
    ];
    let table = Table::new(app.popup_items.content.clone(), widths)
        .widths(widths)
        .block(block)
        .header(header)
        .row_highlight_style(Color::Yellow);
    let rect = centered_rect(55, 45, area);
    frame.render_widget(Clear, rect);
    frame.render_stateful_widget(table, rect, &mut app.popup_items.state.borrow_mut());
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
