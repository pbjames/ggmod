use crate::modz::{LocalCollection, Mod};

pub struct App<'a> {
    collection: &'a mut LocalCollection,
    pub view: View,
    pub search: String,
    pub window: Window,
}

pub enum View {
    Manage,
    Browse,
}

pub enum Window {
    Main,
    Search,
    Category,
}

impl<'a> App<'a> {
    pub fn new(collection: &mut LocalCollection) -> App {
        App {
            collection,
            view: View::Manage,
            search: String::new(),
            window: Window::Main,
        }
    }

    pub fn toggle_view(&mut self) {
        match self.view {
            View::Manage => self.view = View::Browse,
            View::Browse => self.view = View::Manage,
        }
    }

    pub fn search(&mut self) {
        // filter a view of mod key and values
    }
}
