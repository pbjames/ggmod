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

#[derive(Clone)]
pub enum ViewDir {
    Left,
    Right,
}

#[derive(Clone)]
pub enum View {
    Manage(ViewDir),
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
    collection: &'a mut LocalCollection,
    cat_cache: RefCell<Vec<String>>,
    pub view: View,
    pub online_items: ItemizedState<GBSearchEntry>,
    pub staged_items: ItemizedState<&'a Mod>,
    pub unstaged_items: ItemizedState<&'a Mod>,
    pub categories: ItemizedState<GBModCategory>,
    pub sort: CyclicState<FeedFilterIter, FeedFilter>,
    pub section: CyclicState<TypeFilterIter, TypeFilter>,
    pub window: CyclicState<WindowIter, Window>,
    pub page: usize,
}

impl<'a> App<'a> {
    pub fn new(collection: &mut LocalCollection) -> App {
        App {
            collection,
            view: View::Manage(ViewDir::Left),
            cat_cache: RefCell::new(vec![]),
            online_items: ItemizedState::new(),
            categories: ItemizedState::new(),
            staged_items: ItemizedState::new(),
            unstaged_items: ItemizedState::new(),
            section: CyclicState::new(TypeFilter::iter(), TypeFilter::Skin),
            window: CyclicState::new(Window::iter(), Window::Search),
            sort: CyclicState::new(FeedFilter::iter(), FeedFilter::Recent),
            page: 0,
        }
    }

    // TODO: This code is straight ass, all of it
    pub fn next(&mut self) {
        match self.view.clone() {
            View::Manage(dir) => match dir {
                ViewDir::Left => self.staged_items.next(),
                ViewDir::Right => self.unstaged_items.next(),
            },
            View::Browse => self.online_items.next(),
        }
    }

    pub fn previous(&mut self) {
        match self.view.clone() {
            View::Manage(dir) => match dir {
                ViewDir::Left => self.staged_items.previous(),
                ViewDir::Right => self.unstaged_items.previous(),
            },
            View::Browse => self.online_items.previous(),
        }
    }

    pub fn type_search(&mut self, c: char) {
        match self.view.clone() {
            View::Manage(dir) => match dir {
                ViewDir::Left => self.staged_items.query.push(c),
                ViewDir::Right => self.unstaged_items.query.push(c),
            },
            View::Browse => self.online_items.query.push(c),
        }
    }

    pub fn backspace(&mut self) {
        match self.view.clone() {
            View::Manage(dir) => match dir {
                ViewDir::Left => self.staged_items.query.pop(),
                ViewDir::Right => self.unstaged_items.query.pop(),
            },
            View::Browse => self.online_items.query.pop(),
        };
    }

    pub fn search_query(&self) -> String {
        match self.view.clone() {
            View::Manage(dir) => match dir {
                ViewDir::Left => self.staged_items.query.clone(),
                ViewDir::Right => self.unstaged_items.query.clone(),
            },
            View::Browse => self.online_items.query.clone(),
        }
    }

    pub fn search_items(&self) -> Vec<ListItem> {
        match self.view.clone() {
            View::Manage(dir) => match dir {
                ViewDir::Left => self.staged_items.items(),
                ViewDir::Right => self.unstaged_items.items(),
            },
            View::Browse => self.online_items.items(),
        }
    }

    pub fn search_state(&self) -> &RefCell<ListState> {
        match self.view.clone() {
            View::Manage(dir) => match dir {
                ViewDir::Left => &self.staged_items.state,
                ViewDir::Right => &self.unstaged_items.state,
            },
            View::Browse => &self.online_items.state,
        }
    }

    pub fn toggle_view(&mut self) {
        if let Window::Main = self.window.item {
            self.view = match self.view {
                View::Manage(_) => View::Browse,
                View::Browse => View::Manage(ViewDir::Left),
            }
        }
    }

    pub fn select(&mut self) {
        // TODO: Redo
        match self.view.clone() {
            View::Manage(dir) => match dir {
                ViewDir::Left => {
                    if let Some(m) = self.staged_items.select() {
                        self.collection.toggle(m.id).unwrap();
                    }
                }
                ViewDir::Right => {
                    if let Some(m) = self.unstaged_items.select() {
                        self.collection.toggle(m.id).unwrap();
                    }
                }
            },
            View::Browse => {
                if let Some(entry) = self.online_items.select() {
                    self.collection
                        .register_online_mod(entry.mod_page().unwrap(), 0)
                        .unwrap();
                }
            }
        }
    }

    pub fn search(&mut self) -> Result<()> {
        // TODO: Make page size = term height
        match self.view {
            View::Manage(_) => Ok(()),
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

    pub fn toggle_sides(&mut self) {
        match self.view {
            View::Manage(ViewDir::Left) => self.view = View::Manage(ViewDir::Right),
            View::Manage(ViewDir::Right) => self.view = View::Manage(ViewDir::Left),
            View::Browse => (),
        }
    }
}
