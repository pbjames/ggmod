use core::fmt;

use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::Block,
};

use crate::tui::app::{App, Window};

pub fn hide_unfocused<'a>(widget: Block<'a>, app: &App, window: Window) -> Block<'a> {
    widget.style(Style::default().fg(if app.window.item == window {
        Color::White
    } else {
        Color::DarkGray
    }))
}

pub fn enum_to_span<'a, T: PartialEq + fmt::Debug>(t: T, app_value: T) -> Span<'a> {
    let span = Span::from(format!("{:?}", t));
    span.style(Style::default().bg(if app_value == t {
        Color::White
    } else {
        Color::DarkGray
    }))
}
