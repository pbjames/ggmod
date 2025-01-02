use std::iter::Cycle;

use std::cell::RefCell;

use log::trace;
use ordermap::OrderMap;
use ratatui::widgets::{ListItem, ListState};

use crate::gamebanana::{
    builder::{FeedFilter, SearchBuilder, SearchFilter, TypeFilter},
    models::search_result::GBSearchEntry,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct ItemizedState<T> {
    pub state: RefCell<ListState>,
    pub query: String,
    content: OrderMap<String, T>,
}

impl<T> ItemizedState<T> {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            state: RefCell::new(ListState::default()),
            content: OrderMap::new(),
        }
    }

    pub fn refresh(&mut self, content: OrderMap<String, T>) {
        self.query.clear();
        self.content.clear();
        self.state.borrow_mut().select(None);
        self.content = content;
    }

    pub fn next(&mut self) {
        let n = self.content.len();
        let i = match self.state.borrow().selected() {
            Some(i) => i + (i + 1 < n) as usize,
            None => 0,
        };
        self.state.borrow_mut().select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.borrow().selected() {
            Some(i) => i - (i > 0) as usize,
            None => 0,
        };
        self.state.borrow_mut().select(Some(i));
    }

    pub fn select(&self) -> Option<&T> {
        match self.state.borrow().selected() {
            Some(x) => Some(&self.content[x]),
            None => None,
        }
    }

    pub fn items(&self) -> Vec<ListItem> {
        self.content
            .keys()
            .map(|s| ListItem::new::<&str>(String::as_ref(s)))
            .collect()
    }
}

impl ItemizedState<GBSearchEntry> {
    fn format_entry(entry: &GBSearchEntry) -> String {
        format!("{:<50}: views {}", entry.name, entry.view_count)
    }

    pub fn search(
        &mut self,
        section: TypeFilter,
        sort: FeedFilter,
        category: Option<usize>,
        page: usize,
    ) -> Result<()> {
        let search_type = if let Some(cat_id) = category {
            SearchFilter::Category { cat_id }
        } else if self.query.is_empty() {
            SearchFilter::Game { game_id: 11534 }
        } else {
            SearchFilter::Name {
                search: &self.query,
                game_id: 11534,
            }
        };
        let search = SearchBuilder::new()
            .of_type(section)
            .with_sort(sort)
            .by_search(search_type)
            .of_category(category);
        trace!("Are we searching categorically: {category:?}");
        let results = search.build().read_page(page)?;
        self.refresh(
            results
                .into_iter()
                .map(|entry| (Self::format_entry(&entry), entry))
                .collect(),
        );
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
