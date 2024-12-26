use crate::{
    gamebanana::builder::{FeedFilter, SearchBuilder},
    modz::{LocalCollection, Mod},
};

pub struct App<'a> {
    collection: &'a mut LocalCollection,
    pub sort: FeedFilter,
    pub settings: SearchBuilder<'a>,
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
    Section,
}

impl<'a> App<'a> {
    pub fn new(collection: &mut LocalCollection) -> App {
        App {
            collection,
            view: View::Manage,
            search: String::new(),
            window: Window::Search,
            sort: FeedFilter::Recent,
            settings: SearchBuilder::new(),
        }
    }

    pub fn toggle_view(&mut self) {
        if let Window::Main = self.window {
            match self.view {
                View::Manage => self.view = View::Browse,
                View::Browse => self.view = View::Manage,
            }
        }
    }

    pub fn cycle_window_back(&mut self) {
        match self.window {
            Window::Main => self.window = Window::Search,
            Window::Search => self.window = Window::Section,
            Window::Section => self.window = Window::Category,
            Window::Category => self.window = Window::Main,
        }
    }

    pub fn cycle_window(&mut self) {
        match self.window {
            Window::Main => self.window = Window::Category,
            Window::Category => self.window = Window::Section,
            Window::Section => self.window = Window::Search,
            Window::Search => self.window = Window::Main,
        }
    }

    pub fn cycle_sort(&mut self) {
        match self.sort {
            FeedFilter::Recent => self.sort = FeedFilter::Popular,
            FeedFilter::Popular => self.sort = FeedFilter::Featured,
            FeedFilter::Featured => self.sort = FeedFilter::Recent,
        }
    }

    pub fn cycle_sort_back(&mut self) {
        match self.sort {
            FeedFilter::Featured => self.sort = FeedFilter::Popular,
            FeedFilter::Popular => self.sort = FeedFilter::Recent,
            FeedFilter::Recent => self.sort = FeedFilter::Featured,
        }
    }

    pub fn help_text(&self) -> &str {
        match self.window {
            Window::Main => "h/l to change",
            Window::Category => "j/k scroll",
            Window::Section => "j/k scroll",
            Window::Search => "type to search\n<arrow keys> to sort",
        }
    }

    pub fn search(&mut self) {
        // filter a view of mod key and values
    }

    pub fn scroll_up(&mut self) {}

    pub fn scroll_down(&mut self) {}
}
