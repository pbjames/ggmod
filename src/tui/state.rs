use ordermap::ordermap;
use std::iter::Cycle;

use std::cell::RefCell;

use log::trace;
use ordermap::OrderMap;
use ratatui::widgets::TableState;

use crate::gamebanana::{
    builder::{FeedFilter, SearchBuilder, SearchFilter, TypeFilter},
    models::{category::GBModCategory, file::GBFile, search_result::GBSearchEntry},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait ItemizedState {
    type T;

    fn query(&mut self) -> &mut String;
    fn content(&self) -> &OrderMap<String, Self::T>;
    fn content_mut(&mut self) -> &mut OrderMap<String, Self::T>;
    fn state(&self) -> &RefCell<TableState>;
    fn set_content(&mut self, content: OrderMap<String, Self::T>);
    fn search(
        &mut self,
        section: TypeFilter,
        sort: FeedFilter,
        category: Option<usize>,
        page: usize,
    ) -> Result<()>;

    fn refresh(&mut self, content: OrderMap<String, Self::T>) {
        self.query().clear();
        self.content_mut().clear();
        self.state().borrow_mut().select(None);
        self.set_content(content);
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

    fn values(&self) -> Vec<&Self::T> {
        self.content().values().collect()
    }
}

pub struct OnlineItems {
    pub query: String,
    pub state: RefCell<TableState>,
    pub content: OrderMap<String, GBSearchEntry>,
}

impl OnlineItems {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            state: RefCell::new(TableState::default()),
            content: OrderMap::new(),
        }
    }
}

impl ItemizedState for OnlineItems {
    type T = GBSearchEntry;

    fn query(&mut self) -> &mut String {
        &mut self.query
    }

    fn content(&self) -> &OrderMap<String, Self::T> {
        &self.content
    }

    fn content_mut(&mut self) -> &mut OrderMap<String, Self::T> {
        &mut self.content
    }

    fn state(&self) -> &RefCell<TableState> {
        &self.state
    }

    fn set_content(&mut self, content: OrderMap<String, Self::T>) {
        self.content = content;
    }

    fn search(
        &mut self,
        section: TypeFilter,
        sort: FeedFilter,
        category: Option<usize>,
        page: usize,
    ) -> Result<()> {
        fn format_entry(entry: &GBSearchEntry) -> String {
            format!("{:<50}: views {}", entry.name, entry.view_count)
        }
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
        let results = search.build().read_page(page)?;
        self.refresh(
            results
                .into_iter()
                .map(|entry| (format_entry(&entry), entry))
                .collect(),
        );
        Ok(())
    }
}

pub struct PopupItems {
    pub query: String,
    pub state: RefCell<TableState>,
    pub content: OrderMap<String, GBFile>,
    pub entry: Option<GBSearchEntry>,
}

impl PopupItems {
    pub fn empty() -> Self {
        Self {
            query: String::new(),
            state: RefCell::new(TableState::default()),
            content: ordermap!(),
            entry: None,
        }
    }

    pub fn new(entry: GBSearchEntry) -> Self {
        Self {
            query: String::new(),
            state: RefCell::new(TableState::default()),
            content: entry
                .mod_page()
                .unwrap()
                .files
                .into_iter()
                .map(|f| (format!("{} | {}", f.file, f.description), f))
                .collect(),
            entry: Some(entry),
        }
    }

    pub fn select_idx(&self) -> Option<usize> {
        self.state().borrow().selected()
    }
}

impl ItemizedState for PopupItems {
    type T = GBFile;

    fn query(&mut self) -> &mut String {
        &mut self.query
    }

    fn content(&self) -> &OrderMap<String, Self::T> {
        &self.content
    }

    fn content_mut(&mut self) -> &mut OrderMap<String, Self::T> {
        &mut self.content
    }

    fn state(&self) -> &RefCell<TableState> {
        &self.state
    }

    fn set_content(&mut self, content: OrderMap<String, Self::T>) {
        self.content = content;
    }

    fn search(
        &mut self,
        _section: TypeFilter,
        _sort: FeedFilter,
        _category: Option<usize>,
        _page: usize,
    ) -> Result<()> {
        Ok(())
    }
}

pub struct LocalItems {
    pub query: String,
    pub state: RefCell<TableState>,
    pub content: OrderMap<String, usize>,
}

impl LocalItems {
    pub fn new(content: OrderMap<String, usize>) -> Self {
        Self {
            query: String::new(),
            state: RefCell::new(TableState::default()),
            content,
        }
    }
}

impl ItemizedState for LocalItems {
    type T = usize;

    fn query(&mut self) -> &mut String {
        &mut self.query
    }

    fn content(&self) -> &OrderMap<String, Self::T> {
        &self.content
    }

    fn content_mut(&mut self) -> &mut OrderMap<String, Self::T> {
        &mut self.content
    }

    fn state(&self) -> &RefCell<TableState> {
        &self.state
    }

    fn set_content(&mut self, content: OrderMap<String, Self::T>) {
        self.content = content;
    }

    fn search(
        &mut self,
        _section: TypeFilter,
        _sort: FeedFilter,
        _category: Option<usize>,
        _page: usize,
    ) -> Result<()> {
        Ok(())
    }
}

pub struct Categories {
    pub query: String,
    pub state: RefCell<TableState>,
    pub content: OrderMap<String, GBModCategory>,
}

impl Categories {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            state: RefCell::new(TableState::default()),
            // TODO: Find out where this magic number come from
            content: GBModCategory::build(12914)
                .unwrap_or_default()
                .into_iter()
                .map(|cat: GBModCategory| (cat.name.clone(), cat))
                .collect(),
        }
    }
}

impl ItemizedState for Categories {
    type T = GBModCategory;

    fn query(&mut self) -> &mut String {
        &mut self.query
    }

    fn content(&self) -> &OrderMap<String, Self::T> {
        &self.content
    }

    fn content_mut(&mut self) -> &mut OrderMap<String, Self::T> {
        &mut self.content
    }

    fn state(&self) -> &RefCell<TableState> {
        &self.state
    }

    fn set_content(&mut self, content: OrderMap<String, Self::T>) {
        self.content = content;
    }

    fn search(
        &mut self,
        _section: TypeFilter,
        _sort: FeedFilter,
        _category: Option<usize>,
        _page: usize,
    ) -> Result<()> {
        Ok(())
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
