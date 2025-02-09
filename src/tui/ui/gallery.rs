use log::info;
use ratatui::{layout::Rect, Frame};
use ratatui_image::StatefulImage;

use crate::tui::app::App;

// XXX: Kinda stupid that this is the only function here but everything is designed
// around app state oh well
pub fn try_draw_gallery(frame: &mut Frame<'_>, app: &mut App, area: Rect) {
    if let Some((_, state)) = app.image_states.get_index(app.gallery_page()) {
        info!(
            "gallery_page: {}, images: {}",
            app.gallery_page(),
            app.image_states.len()
        );
        frame.render_stateful_widget(StatefulImage::default(), area, &mut state.borrow_mut());
        frame.set_cursor_position((frame.area().right(), frame.area().bottom()));
    }
}
