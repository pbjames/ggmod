use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use crate::tui::app::{App, View, ViewDir, Window};

pub fn manage_view(frame: &mut Frame, app: &App, area: Rect) {
    let header = Row::new(vec!["Name", "Character", "Variant"]);
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

fn left_table(app: &App, widths: [Constraint; 3]) -> Table<'_> {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("[2]-Staged")
        .style(Style::default().fg(match app.view {
            View::Manage(ViewDir::Left) if app.window.item == Window::Main => Color::White,
            _ => Color::DarkGray,
        }));
    Table::new(app.staged_items.content.clone(), widths)
        .widths(widths)
        .block(block)
        .row_highlight_style(Color::Green)
}

fn right_table(app: &App, widths: [Constraint; 3]) -> Table<'_> {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Unstaged")
        .style(Style::default().fg(match app.view {
            View::Manage(ViewDir::Right) if app.window.item == Window::Main => Color::White,
            _ => Color::DarkGray,
        }));
    Table::new(app.unstaged_items.content.clone(), widths)
        .widths(widths)
        .block(block)
        .row_highlight_style(Color::Green)
}
