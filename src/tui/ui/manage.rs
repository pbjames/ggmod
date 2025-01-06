use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use crate::tui::app::{App, View, ViewDir, Window};

pub fn manage_view(frame: &mut Frame, app: &App, area: Rect) {
    let header = Row::new(vec!["Name", "Character", "Description"]);
    let widths = [
        Constraint::Length(35),
        Constraint::Length(20),
        Constraint::Fill(1),
    ];
    let halves = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    frame.render_stateful_widget(
        left_table(app, widths).header(header.clone()),
        halves[0],
        &mut app.staged_items.state.borrow_mut(),
    );
    frame.render_stateful_widget(
        right_table(app, widths).header(header),
        halves[1],
        &mut app.unstaged_items.state.borrow_mut(),
    );
}

fn left_table<'a>(app: &'a App, widths: [Constraint; 3]) -> Table<'a> {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("[2]-Staged")
        .style(Style::default().fg(match app.view {
            View::Manage(ViewDir::Left) if app.window.item == Window::Main => Color::White,
            _ => Color::DarkGray,
        }));
    Table::new(app.staged_items_repr(), widths)
        .widths(widths)
        .block(block)
        .row_highlight_style(Color::Yellow)
}

fn right_table<'a>(app: &'a App, widths: [Constraint; 3]) -> Table<'a> {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Unstaged")
        .style(Style::default().fg(match app.view {
            View::Manage(ViewDir::Right) if app.window.item == Window::Main => Color::White,
            _ => Color::DarkGray,
        }));
    Table::new(app.unstaged_items_repr(), widths)
        .widths(widths)
        .block(block)
        .row_highlight_style(Color::Yellow)
}
