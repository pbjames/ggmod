use std::cell::RefCell;

use log::info;
use ordermap::ordermap;
use ratatui::{
    style::{Color, Style},
    widgets::{Row, TableState},
};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    gamebanana::{
        builder::{FeedFilter, FeedFilterIter, TypeFilter, TypeFilterIter},
        models::search_result::GBSearchEntry,
    },
    modz::LocalCollection,
};

use super::state::{Categories, CyclicState, ItemizedState, LocalItems, OnlineItems, PopupItems};

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
    // TODO: Fix dumb
    pub popup_items: PopupItems,
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
            popup_items: PopupItems::empty(),
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

    pub fn open_popup(&mut self, entry: GBSearchEntry) {
        self.popup_items = PopupItems::new(entry)
    }

    pub fn reregister(&mut self) {
        let staged = self.collection.staged_mods();
        let unstaged = self.collection.unstaged_mods();
        self.staged_items.refresh(staged);
        self.unstaged_items.refresh(unstaged);
    }

    pub fn local_items_mut(&mut self, dir: ViewDir) -> &mut LocalItems {
        match dir {
            ViewDir::Left => &mut self.staged_items,
            ViewDir::Right => &mut self.unstaged_items,
        }
    }

    pub fn local_items(&self, dir: ViewDir) -> &LocalItems {
        match dir {
            ViewDir::Left => &self.staged_items,
            ViewDir::Right => &self.unstaged_items,
        }
    }

    // TODO: This code is straight ass, all of it
    pub fn next(&mut self) {
        match self.view.clone() {
            View::Manage(dir) => self.local_items_mut(dir).next(),
            View::Browse => self.online_items.next(),
        }
    }

    pub fn previous(&mut self) {
        match self.view.clone() {
            View::Manage(dir) => self.local_items_mut(dir).previous(),
            View::Browse => self.online_items.previous(),
        }
    }

    pub fn type_search(&mut self, c: char) {
        match self.view.clone() {
            View::Manage(dir) => self.local_items_mut(dir).query.push(c),
            View::Browse => self.online_items.query.push(c),
        }
    }

    pub fn backspace(&mut self) {
        match self.view.clone() {
            View::Manage(dir) => self.local_items_mut(dir).query.pop(),
            View::Browse => self.online_items.query.pop(),
        };
    }

    pub fn search_query(&self) -> String {
        match self.view.clone() {
            View::Manage(dir) => self.local_items(dir).query.clone(),
            View::Browse => self.online_items.query.clone(),
        }
    }

    pub fn search_state(&self) -> &RefCell<TableState> {
        match self.view.clone() {
            View::Manage(dir) => &self.local_items(dir).state,
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
        if !self.popup_items.is_empty() {
            if let Some(idx) = self.popup_items.select_idx() {
                let entry = self.popup_items.entry.clone();
                self.collection
                    .register_online_mod(entry.unwrap().mod_page().unwrap(), idx)
                    .unwrap();
                self.popup_items = PopupItems::empty();
                return;
            }
        }
        match self.view.clone() {
            View::Manage(dir) => {
                if let Some(m) = self.local_items(dir).select() {
                    self.collection.toggle(*m).unwrap();
                }
            }
            View::Browse => {
                if let Some(entry) = self.online_items.select() {
                    let other = entry.clone();
                    self.open_popup(entry.clone());
                    info!("Popup open {:?}", other);
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
            Window::Category => "\nj/k - scroll",
            Window::Section => "\nj/k - scroll",
            Window::Search => {
                "\ntype to search\n\
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
                    Style::default().fg(Color::LightRed)
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
                    Style::default().fg(Color::LightRed)
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
                    Style::default().fg(Color::LightRed)
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

    pub fn popup_items_repr(&self) -> Vec<Row> {
        self.popup_items
            .values()
            .iter()
            .map(|file| {
                Row::new(vec![
                    file.file.clone(),
                    file.description.clone(),
                    file.download_count.to_string(),
                ])
            })
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
    // TODO: Mod variant popup + search bar listens to 1 2 3 4 when empty + toasts
}
