mod util;
use util::*;
mod browse;
mod category;
mod help;
mod manage;
mod popup;
mod search;
mod section;

use crate::tui::ui::help::help_window;
use std::rc::Rc;

use browse::browse_view;
use category::category;
use manage::manage_view;
use popup::popup;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use search::search_bar;
use section::section;

use super::{
    app::{App, View},
    state::ItemizedState,
};

pub fn show_ui(frame: &mut Frame, app: &mut App) {
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
    if !app.popup_items.is_empty() {
        popup(frame, app, view_and_side[0]);
    }
}

fn side_render(frame: &mut Frame, app: &mut App, area: Rc<[Rect]>) {
    help_window(frame, app, area[0]);
    category(frame, app, area[1]);
    section(frame, app, area[2]);
}

fn view_render(frame: &mut Frame, app: &mut App, area: Rc<[Rect]>) {
    search_bar(frame, app, area[0]);
    match app.view {
        View::Manage(_) => manage_view(frame, app, area[1]),
        View::Browse => browse_view(frame, app, area[1]),
    }
}
