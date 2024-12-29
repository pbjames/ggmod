use std::{collections::HashMap, io};

use crate::{
    gamebanana::{
        builder::{FeedFilter, SearchBuilder, SearchFilter, TypeFilter},
        models::search_result::GBSearchEntry,
    },
    modz::{LocalCollection, Mod},
};

pub struct App<'a> {
    collection: &'a LocalCollection,
    pub search_content: HashMap<String, GBSearchEntry>,
    pub page: usize,
    pub sort: FeedFilter,
    pub section: TypeFilter,
    pub view: View,
    pub search_query: String,
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

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn format_entry(entry: &GBSearchEntry) -> String {
    format!("{:<50}: views {}", entry.name, entry.view_count)
}

impl<'a> App<'a> {
    pub fn new(collection: &LocalCollection) -> App {
        App {
            collection,
            view: View::Manage,
            search_content: HashMap::new(),
            search_query: String::new(),
            section: TypeFilter::Mod,
            page: 0,
            window: Window::Search,
            sort: FeedFilter::Recent,
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

    pub fn cycle_section(&mut self) {
        match self.section {
            TypeFilter::Mod => self.section = TypeFilter::WiP,
            TypeFilter::WiP => self.section = TypeFilter::Sound,
            TypeFilter::Sound => self.section = TypeFilter::Mod,
        }
    }

    pub fn cycle_section_back(&mut self) {
        match self.section {
            TypeFilter::Mod => self.section = TypeFilter::Sound,
            TypeFilter::Sound => self.section = TypeFilter::WiP,
            TypeFilter::WiP => self.section = TypeFilter::Mod,
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

    pub fn staged(&self) -> Vec<&str> {
        self.collection
            .filtered(Box::new(|m: &&Mod| m.staged))
            .iter()
            .map(|m: &&Mod| String::as_ref(&m.name))
            .collect()
    }

    pub fn unstaged(&self) -> Vec<&str> {
        self.collection
            .filtered(Box::new(|m: &&Mod| !m.staged))
            .iter()
            .map(|m: &&Mod| String::as_ref(&m.name))
            .collect()
    }

    pub fn search(&mut self) -> Result<()> {
        // TODO: Convert to table format
        let search_type = if self.search_query.is_empty() {
            SearchFilter::Game { game_id: 11534 }
        } else {
            SearchFilter::Name {
                search: &self.search_query,
                game_id: 11534,
            }
        };
        let search = SearchBuilder::new()
            .of_type(self.section.clone())
            .with_sort(self.sort.clone())
            .by_search(search_type);
        let results = search.build().read_page(0)?;
        self.search_content.clear();
        self.search_content = results
            .into_iter()
            .map(|entry| (format_entry(&entry), entry))
            .collect();
        Ok(())
    }
}
