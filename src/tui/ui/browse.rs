use std::{cell::RefCell, path::PathBuf};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};
use ratatui_image::{picker::Picker, StatefulImage};

use crate::tui::{
    app::{App, Window},
    state::ItemizedState,
};

use anyhow::Result;

use super::{hide_unfocused, image_support};

pub fn browse_view(frame: &mut Frame, app: &mut App, area: Rect) {
    let widths = [
        Constraint::Length(35),
        Constraint::Length(20),
        Constraint::Max(6),
        Constraint::Max(6),
        Constraint::Max(10),
        Constraint::Fill(1),
    ];
    let browse_and_image = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(30), Constraint::Fill(1)])
        .split(area);
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
    // INFO: Note that we clone since it's almost equivalent to cloning each string
    // inside each struct
    let text = Table::new(app.online_items.content.clone(), widths)
        .header(header)
        .widths(widths)
        .block(block)
        .row_highlight_style(Style::default().bg(Color::DarkGray));
    frame.render_stateful_widget(
        text,
        if image_support() {
            browse_and_image[0]
        } else {
            area
        },
        &mut app.search_state().borrow_mut(),
    );
    if image_support() {
        if let Some(idx) = app.online_items.select() {
            let downloaded_media: Vec<PathBuf> = idx
                .preview_media
                .clone()
                .into_iter()
                .map(|m| m.fetch())
                .filter_map(Result::ok)
                .collect();
            let mut picker = Picker::from_fontsize((8, 12));
            for path in downloaded_media {
                let orig = StatefulImage::new(None);
                let dyn_img = image::ImageReader::open(path.clone())
                    .unwrap()
                    .decode()
                    .unwrap();
                let image = RefCell::new(picker.new_resize_protocol(dyn_img));
                app.image_states.insert(path.clone(), image);
                frame.render_stateful_widget(
                    orig,
                    browse_and_image[1],
                    &mut app.image_states.get(&path).unwrap().borrow_mut(),
                );
            }
        }
    }
}
