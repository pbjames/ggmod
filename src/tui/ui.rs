use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};

use crate::gamebanana::builder::FeedFilter;

use super::app::{App, View, Window};

pub fn ui(frame: &mut Frame, app: &App) {
    let view_and_side = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Percentage(25)])
        .split(frame.area());
    let view_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(view_and_side[0]);
    let side_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Fill(2),
            Constraint::Fill(1),
        ])
        .split(view_and_side[1]);
    view_render(frame, app, view_chunks);
    side_render(frame, app, side_chunks);
}

fn side_render(frame: &mut Frame, app: &App, area: Rc<[Rect]>) {
    help_window(frame, app, area[0]);
    category(frame, app, area[1]);
    section(frame, app, area[2]);
}

fn help_window(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("[?]-Help")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));
    let text = Paragraph::new(String::from("\n") + app.help_text())
        .block(block)
        .centered();
    frame.render_widget(text, area);
}

fn section(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("[4]-Section")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if let Window::Section = app.window {
            Color::White
        } else {
            Color::Gray
        }));
    let text = Paragraph::new(app.search.clone())
        .block(block)
        .left_aligned();
    frame.render_widget(text, area);
}

fn category(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("[3]-Category")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if let Window::Category = app.window {
            Color::White
        } else {
            Color::Gray
        }));
    let text = Paragraph::new(app.search.clone())
        .block(block)
        .left_aligned();
    frame.render_widget(text, area);
}

fn search_bar(frame: &mut Frame, app: &App, area: Rect) {
    let search_and_sort = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(30)])
        .split(area);
    let block = Block::default()
        .title("[1]-Search")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if let Window::Search = app.window {
            Color::White
        } else {
            Color::Gray
        }));
    let block2 = Block::default()
        .title("Sort")
        .borders(Borders::LEFT.complement())
        .border_style(Style::default().fg(if let Window::Search = app.window {
            Color::White
        } else {
            Color::Gray
        }));
    let search = Paragraph::new(app.search.clone())
        .block(block.clone())
        .left_aligned();
    let sorts = Paragraph::new(
        Line::from_iter([
            Span::styled(
                "Recent",
                Style::default().bg(if let FeedFilter::Recent = app.sort {
                    Color::DarkGray
                } else {
                    Color::Black
                }),
            ),
            Span::raw(" | "),
            Span::styled(
                "Popular",
                Style::default().bg(if let FeedFilter::Popular = app.sort {
                    Color::DarkGray
                } else {
                    Color::Black
                }),
            ),
            Span::raw(" | "),
            Span::styled(
                "Featured",
                Style::default().bg(if let FeedFilter::Featured = app.sort {
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

fn view_render(frame: &mut Frame, app: &App, area: Rc<[Rect]>) {
    search_bar(frame, app, area[0]);
    match app.view {
        View::Manage => manage_view(frame, app, area[1]),
        View::Browse => browse_view(frame, app, area[1]),
    }
}

fn manage_view(frame: &mut Frame, app: &App, area: Rect) {
    let halves = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    let block = Block::default()
        .borders(Borders::ALL)
        .title("[2]-Manage-Mods")
        .border_style(Style::default().fg(if let Window::Main = app.window {
            Color::White
        } else {
            Color::Gray
        }));
    let left = List::new(app.staged()).block(block.clone());
    let right = List::new(app.unstaged()).block(block.clone());
    frame.render_widget(block, area);
    frame.render_widget(left, halves[0]);
    frame.render_widget(right, halves[1]);
}

fn browse_view(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("[2]-Browse")
        .border_style(Style::default().fg(if let Window::Main = app.window {
            Color::White
        } else {
            Color::Gray
        }));
    let text = Paragraph::new("Browse internet")
        .block(block)
        .left_aligned();
    frame.render_widget(text, area);
}
