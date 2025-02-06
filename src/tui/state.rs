use std::iter::Cycle;

use std::cell::RefCell;

use log::trace;
use ratatui::widgets::{Row, TableState};

use crate::{
    gamebanana::{
        builder::{FeedFilter, SearchBuilder, SearchFilter, TypeFilter},
        models::{category::GBModCategory, file::GBFile, search_result::GBSearchEntry},
    },
    modz::Mod,
};

use anyhow::Result;

pub trait Itemized {
    // TODO: Needs optimization since we keep converting and cloning rows
    type T: for<'a> Into<Row<'a>>;

    fn content(&self) -> &Vec<Self::T>;
    fn content_mut(&mut self) -> &mut Vec<Self::T>;
    fn state(&self) -> &RefCell<TableState>;

    fn refresh(&mut self, content: Vec<Self::T>) {
        self.content_mut().clear();
        self.state()
            .borrow_mut()
            .select(if !content.is_empty() { Some(0) } else { None });
        content.into_iter().for_each(|v| self.content_mut().push(v));
    }

    fn clear(&mut self) {
        self.refresh(Vec::new())
    }

    fn is_empty(&self) -> bool {
        self.content().is_empty()
    }

    fn next(&mut self) {
        let n = self.content().len();
        let i = match self.state().borrow().selected() {
            Some(i) => i + (i + 1 < n) as usize,
            None => 0,
        };
        self.state().borrow_mut().select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state().borrow().selected() {
            Some(i) => i - (i > 0) as usize,
            None => 0,
        };
        self.state().borrow_mut().select(Some(i));
    }

    fn select(&self) -> Option<&Self::T> {
        let state = self.state().borrow();
        state.selected().map(|x| &self.content()[x])
    }
}

#[derive(Default)]
pub struct OnlineItems {
    pub query: String,
    pub state: RefCell<TableState>,
    pub content: Vec<GBSearchEntry>,
}

impl OnlineItems {
    pub async fn search(
        &mut self,
        section: TypeFilter,
        sort: FeedFilter,
        category: Option<usize>,
        page: usize,
    ) -> Result<()> {
        let search_type = match category {
            Some(cat_id) if cat_id != 0 => SearchFilter::Category { cat_id },
            Some(_) | None => {
                if self.query.is_empty() {
                    SearchFilter::Game { game_id: 11534 }
                } else {
                    SearchFilter::Name {
                        search: &self.query,
                        game_id: 11534,
                    }
                }
            }
        };
        let search = SearchBuilder::new()
            .of_type(section)
            .with_sort(sort)
            .by_search(search_type)
            .of_category(category.filter(|id| *id != 0));
        trace!("Are we searching categorically: {category:?}");
        let results = search.build().read_page(page).await?;
        self.refresh(results);
        self.query.clear();
        Ok(())
    }
}

impl Itemized for OnlineItems {
    type T = GBSearchEntry;

    fn content(&self) -> &Vec<Self::T> {
        &self.content
    }

    fn content_mut(&mut self) -> &mut Vec<Self::T> {
        &mut self.content
    }

    fn state(&self) -> &RefCell<TableState> {
        &self.state
    }
}

#[derive(Default)]
pub struct PopupItems {
    pub state: RefCell<TableState>,
    pub content: Vec<GBFile>,
    pub entry: Option<GBSearchEntry>,
}

impl PopupItems {
    pub async fn new(entry: GBSearchEntry) -> Self {
        Self {
            state: RefCell::new(TableState::default()),
            content: entry.mod_page().await.unwrap().files,
            entry: Some(entry),
        }
    }

    pub fn select_idx(&self) -> Option<usize> {
        self.state().borrow().selected()
    }
}

impl Itemized for PopupItems {
    type T = GBFile;

    fn content(&self) -> &Vec<Self::T> {
        &self.content
    }

    fn content_mut(&mut self) -> &mut Vec<Self::T> {
        &mut self.content
    }

    fn state(&self) -> &RefCell<TableState> {
        &self.state
    }
}

pub struct LocalItems {
    pub query: String,
    pub state: RefCell<TableState>,
    pub content: Vec<Mod>,
}

impl LocalItems {
    pub fn new(content: Vec<Mod>) -> Self {
        Self {
            query: String::new(),
            state: RefCell::new(TableState::default()),
            content,
        }
    }
}

impl Itemized for LocalItems {
    type T = Mod;

    fn content(&self) -> &Vec<Self::T> {
        &self.content
    }

    fn content_mut(&mut self) -> &mut Vec<Self::T> {
        &mut self.content
    }

    fn state(&self) -> &RefCell<TableState> {
        &self.state
    }
}

pub struct Categories {
    pub state: RefCell<TableState>,
    pub content: Vec<GBModCategory>,
}

impl Categories {
    pub async fn new() -> Self {
        let this = Self {
            state: RefCell::new(TableState::default()),
            // TODO: Find out where this magic number come from
            content: GBModCategory::build(12914).await.unwrap_or_default(),
        };
        this.state().borrow_mut().select(Some(0));
        this
    }
}

impl Itemized for Categories {
    type T = GBModCategory;

    fn content(&self) -> &Vec<Self::T> {
        &self.content
    }

    fn content_mut(&mut self) -> &mut Vec<Self::T> {
        &mut self.content
    }

    fn state(&self) -> &RefCell<TableState> {
        &self.state
    }
}

pub struct CyclicState<I, T> {
    cycle: Cycle<I>,
    len: usize,
    pub item: T,
}

impl<I, T> CyclicState<I, T>
where
    I: Clone + Iterator<Item = T>,
    T: PartialEq,
{
    pub fn new(iter: I, item: T) -> Self {
        let mut cycle = iter.clone().cycle();
        cycle.next();
        Self {
            cycle,
            item,
            len: iter.count(),
        }
    }

    pub fn cycle_to(&mut self, item: T) {
        while self.item != item {
            self.cycle();
        }
    }

    pub fn cycle(&mut self) {
        self.item = self.cycle.next().unwrap()
    }

    pub fn cycle_back(&mut self) {
        for _ in 0..self.len - 2 {
            self.cycle.next();
        }
        self.cycle();
    }
}
