use std::cell::RefCell;

use log::trace;
use ordermap::ordermap;
use ratatui::{
    style::{Color, Style},
    widgets::{Row, TableState},
};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    gamebanana::builder::{FeedFilter, FeedFilterIter, TypeFilter, TypeFilterIter},
    modz::LocalCollection,
};

use super::state::{Categories, CyclicState, ItemizedState, LocalItems, OnlineItems};

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
    pub view: View,
    pub online_items: OnlineItems,
    pub staged_items: LocalItems,
    pub unstaged_items: LocalItems,
    pub categories: Categories,
    pub sort: CyclicState<FeedFilterIter, FeedFilter>,
    pub section: CyclicState<TypeFilterIter, TypeFilter>,
    pub window: CyclicState<WindowIter, Window>,
    pub page: usize,
}

impl<'a> App<'a> {
    pub fn new(collection: &'a mut LocalCollection) -> App<'a> {
        let mut this = App {
            collection,
            view: View::Manage(ViewDir::Left),
            online_items: OnlineItems::new(),
            categories: Categories::new(),
            staged_items: LocalItems::new(ordermap! {}),
            unstaged_items: LocalItems::new(ordermap! {}),
            section: CyclicState::new(TypeFilter::iter(), TypeFilter::Skin),
            window: CyclicState::new(Window::iter(), Window::Search),
            sort: CyclicState::new(FeedFilter::iter(), FeedFilter::Recent),
            page: 0,
        };
        this.reregister();
        this
    }

    pub fn reregister(&mut self) {
        let staged = self.collection.staged_mods();
        let unstaged = self.collection.unstaged_mods();
        self.staged_items.refresh(staged);
        self.unstaged_items.refresh(unstaged);
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

    pub fn search_state(&self) -> &RefCell<TableState> {
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
        trace!("Doth been called");
        match self.view.clone() {
            View::Manage(dir) => match dir {
                ViewDir::Left => {
                    if let Some(m) = self.staged_items.select() {
                        self.collection.toggle(*m).unwrap();
                    }
                }
                ViewDir::Right => {
                    if let Some(m) = self.unstaged_items.select() {
                        self.collection.toggle(*m).unwrap();
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
        self.reregister();
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
                "Space - Install / Uninstall from game dir\n\
                 H / L - Switch local/gamebanana mods\n\
                 h / l - local - Switch sides\n\
                         online - Scroll pages"
            }
            Window::Category => "j/k - scroll",
            Window::Section => "j/k - scroll",
            Window::Search => {
                "\
                type to search\n\
                <arrow keys> to sort by different stuff"
            }
        }
    }

    pub fn toggle_sides(&mut self) {
        match self.view {
            View::Manage(ViewDir::Left) => self.view = View::Manage(ViewDir::Right),
            View::Manage(ViewDir::Right) => self.view = View::Manage(ViewDir::Left),
            View::Browse => (),
        }
    }

    // TODO: This sux
    pub fn unstaged_items_repr(&self) -> Vec<Row> {
        self.unstaged_items
            .values()
            .iter()
            .map(|id| {
                let (name, char, desc, nsfw) = self.mod_id_translate(**id);
                Row::new(vec![name, desc, char]).style(if nsfw {
                    Style::default().bg(Color::LightRed)
                } else {
                    Style::default()
                })
            })
            .collect()
    }

    pub fn staged_items_repr(&self) -> Vec<Row> {
        self.staged_items
            .values()
            .iter()
            .map(|id| {
                let (name, char, desc, nsfw) = self.mod_id_translate(**id);
                Row::new(vec![name, desc, char]).style(if nsfw {
                    Style::default().bg(Color::LightRed)
                } else {
                    Style::default()
                })
            })
            .collect()
    }

    pub fn online_items_repr(&self) -> Vec<Row> {
        self.online_items
            .values()
            .iter()
            .map(|ent| {
                Row::new(vec![
                    ent.name.clone(),
                    ent.category.name.clone(),
                    ent.view_count.to_string(),
                    ent.like_count.to_string(),
                    ent.download_count.to_string(),
                    ent.description.clone(),
                ])
                .style(if ent.is_nsfw {
                    Style::default().bg(Color::LightRed)
                } else {
                    Style::default()
                })
            })
            .collect()
    }

    pub fn categories_repr(&self) -> Vec<Row> {
        self.categories
            .values()
            .iter()
            .map(|cat| Row::new(vec![cat.name.clone()]))
            .collect()
    }

    pub fn mod_id_translate(&self, id: usize) -> (String, String, String, bool) {
        self.collection
            .mods()
            .iter()
            .filter(|m| m.id == id)
            .map(|m| {
                (
                    m.name.clone(),
                    m.description.clone(),
                    m.character.clone(),
                    m.is_nsfw,
                )
            })
            .next()
            .unwrap()
    }
}
