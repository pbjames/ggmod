use core::fmt::Debug;

use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::Block,
};
use strum::IntoEnumIterator;

use crate::tui::{
    app::{App, Window},
    state::ItemizedState,
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

pub fn enum_to_span<'a, T>(app_value: T, repr: Box<dyn Fn(T) -> String>) -> Vec<Span<'a>>
where
    T: PartialEq + Debug + IntoEnumIterator + Clone,
{
    T::iter()
        .map(|t| {
            Span::from(repr(t.clone())).style(Style::default().fg(if app_value == t {
                Color::LightBlue
            } else {
                Color::Gray
            }))
        })
        .collect()
}
