use std::cell::RefCell;

use log::debug;
use ratatui::widgets::{ListItem, ListState};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    gamebanana::{
        builder::{FeedFilter, FeedFilterIter, TypeFilter, TypeFilterIter},
        models::{category::GBModCategory, search_result::GBSearchEntry},
    },
    modz::{LocalCollection, Mod},
};

use super::state::{CyclicState, ItemizedState};

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
    cat_cache: RefCell<Vec<String>>,
    pub view: View,
    pub online_items: ItemizedState<GBSearchEntry>,
    pub local_items: ItemizedState<&'a Mod>,
    pub categories: ItemizedState<GBModCategory>,
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
            cat_cache: RefCell::new(vec![]),
            online_items: ItemizedState::new(),
            categories: ItemizedState::new(),
            local_items: ItemizedState::new(),
            section: CyclicState::new(TypeFilter::iter(), TypeFilter::Mod),
            window: CyclicState::new(Window::iter(), Window::Search),
            sort: CyclicState::new(FeedFilter::iter(), FeedFilter::Recent),
            page: 0,
        }
    }

    pub fn next(&mut self) {
        match self.view {
            View::Manage => self.local_items.next(),
            View::Browse => self.online_items.next(),
        }
    }

    pub fn previous(&mut self) {
        match self.view {
            View::Manage => self.local_items.previous(),
            View::Browse => self.online_items.previous(),
        }
    }

    pub fn type_search(&mut self, c: char) {
        match self.view {
            View::Manage => self.local_items.query.push(c),
            View::Browse => self.online_items.query.push(c),
        }
    }

    pub fn backspace(&mut self) {
        match self.view {
            View::Manage => self.local_items.query.pop(),
            View::Browse => self.online_items.query.pop(),
        };
    }

    pub fn search_query(&self) -> String {
        match self.view {
            View::Manage => self.local_items.query.clone(),
            View::Browse => self.online_items.query.clone(),
        }
    }

    pub fn search_items(&self) -> Vec<ListItem> {
        match self.view {
            View::Manage => self.local_items.items(),
            View::Browse => self.online_items.items(),
        }
    }

    pub fn search_state(&self) -> &RefCell<ListState> {
        match self.view {
            View::Manage => &self.local_items.state,
            View::Browse => &self.online_items.state,
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
            View::Browse => self.online_items.search(
                self.section.item.clone(),
                self.sort.item.clone(),
                self.categories.select().map(|cat| cat.row),
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

    pub fn categories(&mut self) -> Vec<String> {
        if self.cat_cache.borrow().is_empty() {
            let cats = GBModCategory::build(12914).unwrap_or_default();
            let names: Vec<String> = cats.iter().map(|cat| cat.name.clone()).collect();
            self.cat_cache.replace(names.clone());
            self.categories
                .refresh(names.clone().into_iter().zip(cats).collect());
            names
        } else {
            debug!("Cache hit");
            self.cat_cache.borrow().to_vec()
        }
    }
}
