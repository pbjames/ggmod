use std::{cell::RefCell, iter::Cycle};

use ratatui::widgets::{ListItem, ListState};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    gamebanana::{
        builder::{FeedFilter, TypeFilter},
        models::search_result::GBSearchEntry,
    },
    modz::{LocalCollection, Mod},
};

use super::search::Searcher;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum View {
    Manage,
    Browse,
}

#[derive(EnumIter)]
pub enum Window {
    Search,
    Main,
    Category,
    Section,
}

pub struct App<'a> {
    collection: &'a LocalCollection,
    window_cycle: Cycle<WindowIter>,
    pub browse_search: Searcher<GBSearchEntry>,
    pub local_search: Searcher<&'a Mod>,
    pub page: usize,
    pub sort: FeedFilter,
    pub section: TypeFilter,
    pub view: View,
    pub window: Window,
}

impl<'a> App<'a> {
    pub fn new(collection: &LocalCollection) -> App {
        App {
            collection,
            view: View::Manage,
            browse_search: Searcher::new(),
            local_search: Searcher::new(),
            section: TypeFilter::Mod,
            page: 0,
            window: Window::Search,
            window_cycle: Window::iter().cycle(),
            sort: FeedFilter::Recent,
        }
    }

    // TODO: Remove all of this and replace with agnostic getter
    pub fn next(&mut self) {
        match self.view {
            View::Manage => self.local_search.next(),
            View::Browse => self.browse_search.next(),
        }
    }

    pub fn previous(&mut self) {
        match self.view {
            View::Manage => self.local_search.previous(),
            View::Browse => self.browse_search.previous(),
        }
    }

    pub fn type_search(&mut self, c: char) {
        match self.view {
            View::Manage => self.local_search.query.push(c),
            View::Browse => self.local_search.query.push(c),
        }
    }

    pub fn backspace(&mut self) {
        match self.view {
            View::Manage => self.local_search.query.pop(),
            View::Browse => self.local_search.query.pop(),
        };
    }

    pub fn search_query(&self) -> String {
        match self.view {
            View::Manage => self.local_search.query.clone(),
            View::Browse => self.browse_search.query.clone(),
        }
    }

    pub fn search_items(&self) -> Vec<ListItem> {
        match self.view {
            View::Manage => self.local_search.items(),
            View::Browse => self.browse_search.items(),
        }
    }

    pub fn search_state(&self) -> &RefCell<ListState> {
        match self.view {
            View::Manage => &self.local_search.state,
            View::Browse => &self.browse_search.state,
        }
    }

    pub fn toggle_view(&mut self) {
        if let Window::Main = self.window {
            self.view = match self.view {
                View::Manage => View::Browse,
                View::Browse => View::Manage,
            }
        }
    }

    pub fn search(&mut self) -> Result<()> {
        // TODO: Make page size = term height
        match self.view {
            View::Manage => Ok(()),
            View::Browse => {
                self.browse_search
                    .search(self.section.clone(), self.sort.clone(), self.page)
            }
        }
    }

    pub fn cycle_window(&mut self) {
        self.window = self.window_cycle.next().unwrap();
    }

    pub fn cycle_window_back(&mut self) {
        for _ in 0..Window::iter().len() - 1 {
            self.window_cycle.next();
        }
        self.window = self.window_cycle.next().unwrap()
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
}
