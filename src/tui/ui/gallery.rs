use std::{cell::RefCell, path::PathBuf};

use ratatui::{layout::Rect, Frame};
use ratatui_image::{picker::Picker, StatefulImage};

use crate::tui::{app::App, state::Itemized};

pub fn gallery(frame: &mut Frame, app: &mut App, area: Rect) {
    let mut picker = Picker::from_fontsize((8, 12));
    if let Some(entry) = app.online_items.select() {
        let downloaded_media = entry.download_media();
        // XXX: Fucking stupid fix
        while app.gallery_page() >= downloaded_media.len() {
            app.gallery_prev();
        }
        let image_path = downloaded_media.get(app.gallery_page()).unwrap();
        check_insert_image(app, &mut picker, image_path);
        frame.render_stateful_widget(
            StatefulImage::new(None),
            area,
            &mut app.image_states.get(image_path).unwrap().borrow_mut(),
        );
    }
}

fn check_insert_image(app: &mut App, picker: &mut Picker, path: &PathBuf) {
    if !app.image_states.contains_key(path) {
        let dyn_img = image::ImageReader::open(path.clone())
            .unwrap()
            .decode()
            .unwrap();
        let image = RefCell::new(picker.new_resize_protocol(dyn_img));
        app.image_states.insert(path.clone(), image);
    }
}
