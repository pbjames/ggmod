use std::{cell::RefCell, path::PathBuf, rc::Rc};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use ratatui_image::{picker::Picker, StatefulImage};

use crate::tui::{app::App, state::Itemized};

pub fn gallery(frame: &mut Frame, app: &mut App, area: Rect) {
    let mut picker = Picker::from_fontsize((8, 12));
    if let Some(entry) = app.online_items.select() {
        let downloaded_media = entry.download_media();
        let areas = divide_area_horiz(area, downloaded_media.len());
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

fn divide_area_horiz(area: Rect, n: usize) -> Rc<[Rect]> {
    let constraints = vec![Constraint::Fill(1); n];
    Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area)
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
