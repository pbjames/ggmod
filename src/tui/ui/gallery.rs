use log::info;
use ratatui::{layout::Rect, Frame};
use ratatui_image::StatefulImage;

use crate::tui::app::App;

use super::centered_rect;

// XXX: Kinda stupid that this is the only function here but everything is designed
// around app state oh well
pub fn try_draw_gallery(frame: &mut Frame<'_>, app: &mut App, area: Rect) {
    let rect = centered_rect(80, 80, area);
    if let Some((_, state)) = app.image_states.get_index(app.gallery_page()) {
        info!(
            "gallery_page: {}, images: {}",
            app.gallery_page(),
            app.image_states.len()
        );
        frame.render_stateful_widget(StatefulImage::default(), rect, &mut state.borrow_mut());
        frame.set_cursor_position((frame.area().right(), frame.area().bottom()));
    }
}
