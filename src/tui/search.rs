use std::{cell::RefCell, collections::HashMap};

use ratatui::widgets::{ListItem, ListState};

use crate::gamebanana::{
    builder::{FeedFilter, SearchBuilder, SearchFilter, TypeFilter},
    models::search_result::GBSearchEntry,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Searcher<T> {
    pub state: RefCell<ListState>,
    pub query: String,
    content: HashMap<String, T>,
}

impl<T> Searcher<T> {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            state: RefCell::new(ListState::default()),
            content: HashMap::new(),
        }
    }

    pub fn refresh(&mut self, content: HashMap<String, T>) {
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

    pub fn items(&self) -> Vec<ListItem> {
        self.content
            .keys()
            .map(|s| ListItem::new::<&str>(String::as_ref(s)))
            .collect()
    }
}

impl Searcher<GBSearchEntry> {
    fn format_entry(entry: &GBSearchEntry) -> String {
        format!("{:<50}: views {}", entry.name, entry.view_count)
    }

    pub fn search(&mut self, section: TypeFilter, sort: FeedFilter, page: usize) -> Result<()> {
        // TODO: Convert to table format
        let search_type = if self.query.is_empty() {
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
            .by_search(search_type);
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
