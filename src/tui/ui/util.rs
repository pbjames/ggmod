use core::fmt::Debug;
use std::{cell::RefCell, path::PathBuf, rc::Rc};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::Block,
};
use ratatui_image::picker::Picker;
use strum::IntoEnumIterator;

use crate::tui::{
    app::{App, Window},
    state::Itemized,
};

pub fn hide_unfocused<'a>(widget: Block<'a>, app: &App, window: Window) -> Block<'a> {
    widget.style(
        Style::default().fg(if app.window.item == window && app.popup_items.is_empty() {
            Color::White
        } else {
            Color::DarkGray
        }),
    )
}

// INFO: I like this one
pub fn enum_to_span<'a, T>(app_value: T) -> Vec<Span<'a>>
where
    T: PartialEq + Debug + IntoEnumIterator + Clone,
{
    T::iter()
        .map(|t| {
            Span::from(format!("{t:?}")).style(Style::default().fg(if app_value == t {
                Color::LightBlue
            } else {
                Color::Gray
            }))
        })
        .collect()
}

pub fn image_support() -> bool {
    // TODO: Fix extreme lag and finish this
    true
}

pub fn divide_area_horiz(area: Rect, n: usize) -> Rc<[Rect]> {
    let constraints = vec![Constraint::Fill(1); n];
    Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area)
}

pub fn check_insert_image(app: &mut App, picker: &mut Picker, path: &PathBuf) {
    if !app.image_states.contains_key(path) {
        let dyn_img = image::ImageReader::open(path.clone())
            .unwrap()
            .decode()
            .unwrap();
        let image = RefCell::new(picker.new_resize_protocol(dyn_img));
        app.image_states.insert(path.clone(), image);
    }
}
