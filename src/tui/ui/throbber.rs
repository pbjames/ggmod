use crate::tui::app::App;
use log::info;
use ratatui::layout::Margin;
use ratatui::{layout::Rect, Frame};

pub fn try_throbber(frame: &mut Frame, app: &mut App, area: Rect) {
    if let Some(state) = &mut app.throbber_state {
        info!("THROB");
        let right = area
            .inner(Margin {
                horizontal: 1,
                vertical: 1,
            })
            .right();
        let rect: Rect = area.intersection(Rect {
            x: right,
            y: area.y,
            width: 1,
            height: 1,
        });
        let full = throbber_widgets_tui::Throbber::default()
            .label("Running...")
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
            .throbber_style(
                ratatui::style::Style::default()
                    .fg(ratatui::style::Color::Red)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )
            .throbber_set(throbber_widgets_tui::ASCII)
            .use_type(throbber_widgets_tui::WhichUse::Spin);
        frame.render_stateful_widget(full, rect, state);
        app.throb();
    }
}
