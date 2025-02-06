use core::fmt::Debug;

use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::Block,
};
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
    // TODO: Finish this
    false
}
