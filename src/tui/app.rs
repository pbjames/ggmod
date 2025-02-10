use std::{cell::RefCell, path::PathBuf};

use indexmap::IndexMap;
use log::info;
use ratatui::widgets::TableState;
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    gamebanana::{
        builder::{FeedFilter, FeedFilterIter, TypeFilter, TypeFilterIter},
        models::search_result::GBSearchEntry,
    },
    modz::{LocalCollection, Mod},
};

use anyhow::Result;

use super::state::{Categories, CyclicState, Itemized, LocalItems, OnlineItems, PopupItems};

#[derive(Copy, Clone)]
pub enum ViewDir {
    Left,
    Right,
}

#[derive(Copy, Clone)]
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

/// Basically a container that holds state from state.rs and acts differently based on
/// current state beind held
pub struct App {
    collection: LocalCollection,
    page: usize,
    gallery_page: usize,
    pub popup_items: PopupItems,
    pub online_items: OnlineItems,
    pub staged_items: LocalItems,
    pub unstaged_items: LocalItems,
    pub categories: Categories,
    pub section: CyclicState<TypeFilterIter, TypeFilter>,
    pub cursor: Option<usize>,
    pub view: View,
    pub window: CyclicState<WindowIter, Window>,
    pub sort: CyclicState<FeedFilterIter, FeedFilter>,
    pub image_states: IndexMap<PathBuf, RefCell<StatefulProtocol>>,
}

impl App {
    pub async fn new(collection: LocalCollection) -> App {
        let mut this = App {
            collection,
            popup_items: PopupItems::default(),
            online_items: OnlineItems::default(),
            staged_items: LocalItems::new(Vec::new()),
            unstaged_items: LocalItems::new(Vec::new()),
            categories: Categories::new().await,
            section: CyclicState::new(TypeFilter::iter(), TypeFilter::Skin),
            cursor: None,
            view: View::Manage(ViewDir::Left),
            window: CyclicState::new(Window::iter(), Window::Search),
            sort: CyclicState::new(FeedFilter::iter(), FeedFilter::Recent),
            page: 0,
            gallery_page: 0,
            image_states: IndexMap::new(),
        };
        this.reregister();
        this
    }

    pub async fn open_popup(&mut self, entry: GBSearchEntry) {
        self.popup_items = PopupItems::new(entry).await;
        self.request_gallery_images().await;
    }

    pub fn reregister(&mut self) {
        let staged = self.collection.filter_and_copy_by(Box::new(|m| m.staged));
        let unstaged = self.collection.filter_and_copy_by(Box::new(|m| !m.staged));
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

    // INFO: Apparently a necessary evil, even though it's shit it's also quite simple
    pub fn next(&mut self) {
        match self.view {
            View::Manage(dir) => self.local_items_mut(dir).next(),
            View::Browse => self.online_items.next(),
        }
    }

    pub fn previous(&mut self) {
        match self.view {
            View::Manage(dir) => self.local_items_mut(dir).previous(),
            View::Browse => self.online_items.previous(),
        }
    }

    pub fn type_search(&mut self, c: char) {
        match self.view {
            View::Manage(dir) => self.local_items_mut(dir).query.push(c),
            View::Browse => self.online_items.query.push(c),
        }
        self.cursor = Some(self.cursor.map_or(1, |v| v + 1));
    }

    pub fn backspace(&mut self) {
        match self.view {
            View::Manage(dir) => self.local_items_mut(dir).query.pop(),
            View::Browse => self.online_items.query.pop(),
        };
        self.cursor = self.cursor.map(|v| if v > 0 { v - 1 } else { v });
    }

    pub fn search_query(&self) -> String {
        match self.view {
            View::Manage(dir) => self.local_items(dir).query.clone(),
            View::Browse => self.online_items.query.clone(),
        }
    }

    pub fn search_state(&self) -> &RefCell<TableState> {
        match self.view {
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
        self.reset_cursor();
    }

    pub fn toggle_sides(&mut self) {
        match self.view {
            View::Manage(ViewDir::Left) => self.view = View::Manage(ViewDir::Right),
            View::Manage(ViewDir::Right) => self.view = View::Manage(ViewDir::Left),
            View::Browse => (),
        }
    }

    pub async fn select(&mut self) {
        if !self.popup_items.is_empty() {
            if let Some(idx) = self.popup_items.select_idx() {
                let entry = self.popup_items.entry.clone();
                self.collection
                    .register_online_mod(entry.unwrap().mod_page().await.unwrap(), idx)
                    .await
                    .unwrap();
                self.popup_items.clear();
                self.image_states.clear();
                self.reregister();
                return;
            }
        }
        match self.view {
            View::Manage(dir) => {
                if let Some(m) = self.local_items(dir).select() {
                    self.collection.toggle(m.id).unwrap();
                }
            }
            View::Browse => {
                if let Some(entry) = self.online_items.select() {
                    let other = entry.clone();
                    self.open_popup(entry.clone()).await;
                    info!("Popup open {:?}", other);
                }
            }
        }
        self.reregister();
    }

    pub async fn search(&mut self) -> Result<()> {
        // TODO: Make page size = term height
        match self.view {
            View::Manage(_) => Ok(()),
            View::Browse => {
                self.online_items
                    .search(
                        self.section.item.clone(),
                        self.sort.item.clone(),
                        self.categories.select().map(|cat| cat.row),
                        self.page,
                    )
                    .await
            }
        }
    }

    pub fn help_text(&self) -> &str {
        match self.window.item {
            Window::Main => {
                "Space - Install / Uninstall from game dir\n\
                 H / L - Switch local/gamebanana mods\n\
                 h / l - local - Switch sides\n\
                         online - Scroll pages\n\
                 x - local - Delete mod permanently"
            }
            Window::Category => "j/k - scroll",
            Window::Section => "j/k - scroll",
            Window::Search => {
                "type and press enter to search\n\
                 arrow keys to sort by different stuff"
            }
        }
    }

    pub fn gallery_prev(&mut self) {
        if self.gallery_page == 0 {
            return;
        };
        self.gallery_page -= 1;
    }

    pub fn gallery_next(&mut self) {
        if self.gallery_page < self.image_states.len() - 1 {
            self.gallery_page += 1;
        }
    }

    pub fn gallery_page(&self) -> usize {
        self.gallery_page
    }

    pub fn reset_cursor(&mut self) {
        let length = self.search_query().len();
        self.cursor = if length == 0 { None } else { Some(length) }
    }

    pub fn remove(&mut self) -> Option<()> {
        if let View::Manage(dir) = self.view {
            if let Some(chosen) = self.local_items(dir).select() {
                let pred = |m: &Mod| m.id == chosen.id;
                let idx = self.collection.mods.iter().position(pred)?;
                self.collection.mods.remove(idx);
                self.reregister();
            }
        }
        Some(())
    }

    pub async fn request_gallery_images(self: &mut App) {
        let mut picker = Picker::from_fontsize((8, 12));
        if let Some(entry) = self.online_items.select() {
            // TODO: Make this request more images on demand or something
            let downloaded_media = entry.download_media(1).await;
            if let Some(path) = downloaded_media.get(self.gallery_page()) {
                self.check_insert_image(&mut picker, path);
            }
        }
    }

    fn check_insert_image(self: &mut App, picker: &mut Picker, path: &PathBuf) {
        if !self.image_states.contains_key(path) {
            let dyn_img = image::ImageReader::open(path.clone())
                .unwrap()
                .decode()
                .unwrap()
                .resize(400, 400, ratatui_image::FilterType::Gaussian);
            let image = RefCell::new(picker.new_resize_protocol(dyn_img));
            self.image_states.insert(path.clone(), image);
        }
    }

    // TODO: Toasts
    // + Perf. optimsation
    // + Sorting tables
    // + Fix sfw searching + images
    // + Mod deletion and modification
    // + Replace builder with derive_builder macro (long)
}
