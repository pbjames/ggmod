use std::cell::RefCell;

use ratatui::widgets::{ListItem, ListState};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    gamebanana::{
        builder::{FeedFilter, FeedFilterIter, TypeFilter, TypeFilterIter},
        models::search_result::GBSearchEntry,
    },
    modz::{LocalCollection, Mod},
};

use super::{search::Searcher, state::CyclicState};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum View {
    Manage,
    Browse,
}

#[derive(EnumIter, PartialEq, PartialOrd)]
pub enum Window {
    Search,
    Main,
    Category,
    Section,
}

pub struct App<'a> {
    collection: &'a LocalCollection,
    pub view: View,
    pub browse_search: Searcher<GBSearchEntry>,
    pub local_search: Searcher<&'a Mod>,
    pub sort: CyclicState<FeedFilterIter, FeedFilter>,
    pub section: CyclicState<TypeFilterIter, TypeFilter>,
    pub window: CyclicState<WindowIter, Window>,
    pub page: usize,
}

impl<'a> App<'a> {
    pub fn new(collection: &LocalCollection) -> App {
        App {
            collection,
            view: View::Manage,
            browse_search: Searcher::new(),
            local_search: Searcher::new(),
            section: CyclicState::new(TypeFilter::iter(), TypeFilter::Mod),
            window: CyclicState::new(Window::iter(), Window::Search),
            sort: CyclicState::new(FeedFilter::iter(), FeedFilter::Recent),
            page: 0,
        }
    }

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
            View::Browse => self.browse_search.query.push(c),
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
        if let Window::Main = self.window.item {
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
            View::Browse => self.browse_search.search(
                self.section.item.clone(),
                self.sort.item.clone(),
                self.page,
            ),
        }
    }

    pub fn help_text(&self) -> &str {
        match self.window.item {
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
