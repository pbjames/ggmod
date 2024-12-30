use std::{cell::RefCell, collections::HashMap};

use ratatui::widgets::{ListItem, ListState};

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
    pub search_state: RefCell<ListState>,
    pub search_query: String,
    pub window: Window,
}

pub enum View {
    Manage,
    Browse,
}

// TODO: Turn everything using this into way less code
pub enum Window {
    Unfocused,
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
            search_state: RefCell::new(ListState::default()),
            window: Window::Search,
            sort: FeedFilter::Recent,
        }
    }

    pub fn clear_search_state(&mut self) {
        self.search_content.clear();
        self.search_state = RefCell::new(ListState::default());
    }

    pub fn next(&mut self) {
        let n = self.search_content.len();
        let i = match self.search_state.borrow().selected() {
            Some(i) => i + (i + 1 < n) as usize,
            None => 0,
        };
        self.search_state.borrow_mut().select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.search_state.borrow().selected() {
            Some(i) => i - (i > 0) as usize,
            None => 0,
        };
        self.search_state.borrow_mut().select(Some(i));
    }

    pub fn toggle_view(&mut self) {
        if let Window::Main = self.window {
            self.view = match self.view {
                View::Manage => View::Browse,
                View::Browse => View::Manage,
            }
        }
    }

    pub fn cycle_window_back(&mut self) {
        self.window = match self.window {
            Window::Unfocused => Window::Main,
            Window::Main => Window::Search,
            Window::Search => Window::Section,
            Window::Section => Window::Category,
            Window::Category => Window::Main,
        }
    }

    pub fn cycle_window(&mut self) {
        self.window = match self.window {
            Window::Unfocused => Window::Main,
            Window::Main => Window::Category,
            Window::Category => Window::Section,
            Window::Section => Window::Search,
            Window::Search => Window::Main,
        }
    }

    pub fn cycle_sort(&mut self) {
        self.sort = match self.sort {
            FeedFilter::Recent => FeedFilter::Popular,
            FeedFilter::Popular => FeedFilter::Featured,
            FeedFilter::Featured => FeedFilter::Recent,
        }
    }

    pub fn cycle_sort_back(&mut self) {
        self.sort = match self.sort {
            FeedFilter::Featured => FeedFilter::Popular,
            FeedFilter::Popular => FeedFilter::Recent,
            FeedFilter::Recent => FeedFilter::Featured,
        }
    }

    pub fn cycle_section(&mut self) {
        self.section = match self.section {
            TypeFilter::Mod => TypeFilter::WiP,
            TypeFilter::WiP => TypeFilter::Sound,
            TypeFilter::Sound => TypeFilter::Mod,
        }
    }

    pub fn cycle_section_back(&mut self) {
        self.section = match self.section {
            TypeFilter::Mod => TypeFilter::Sound,
            TypeFilter::Sound => TypeFilter::WiP,
            TypeFilter::WiP => TypeFilter::Mod,
        }
    }

    pub fn help_text(&self) -> &str {
        match self.window {
            Window::Unfocused => "q - exit",
            Window::Main => {
                "\
                H/L - Switch local/gamebanana mods\n\
                h/l - local - Switch sides\
                    - online - Scroll pages"
            }
            Window::Category => "j/k - scroll",
            Window::Section => "j/k - scroll",
            Window::Search => {
                "\
                type to search\n\
                <arrow keys> to sort"
            }
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
        let results = search.build().read_page(self.page)?;
        self.clear_search_state();
        self.search_content = results
            .into_iter()
            .map(|entry| (format_entry(&entry), entry))
            .collect();
        Ok(())
    }

    pub fn search_items(&self) -> Vec<ListItem> {
        self.search_content
            .keys()
            .map(|s| ListItem::new::<&str>(String::as_ref(s)))
            .collect()
    }
}
