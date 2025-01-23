use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};
use ratatui_image::{picker::Picker, StatefulImage};

use crate::tui::{
    app::{App, Window},
    state::Itemized,
};

use super::{check_insert_image, divide_area_horiz, hide_unfocused, image_support};

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
        if let Some(entry) = app.online_items.select() {
            let downloaded_media = entry.download_media();
            let areas = divide_area_horiz(browse_and_image[1], downloaded_media.len());
            let mut picker = Picker::from_fontsize((8, 12));
            for (i, path) in downloaded_media.iter().enumerate() {
                check_insert_image(app, &mut picker, path);
                let orig = StatefulImage::new(None);
                frame.render_stateful_widget(
                    orig,
                    areas[i],
                    &mut app.image_states.get(path).unwrap().borrow_mut(),
                );
            }
        }
    }
}
