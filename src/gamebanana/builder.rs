use strum::EnumIter;

use super::search::Search;

#[derive(EnumIter, Clone, Debug, PartialEq)]
pub enum TypeFilter {
    Mod,
    Sound,
    WiP,
}

pub enum SearchFilter<'a> {
    Name { search: &'a str, game_id: usize },
    Game { game_id: usize },
    Category { cat_id: usize },
}

#[derive(EnumIter, Clone, Debug, PartialEq)]
pub enum FeedFilter {
    Recent,
    Popular,
    Featured,
}

pub struct SearchBuilder<'a> {
    mod_type: TypeFilter,
    search: SearchFilter<'a>,
    feed: FeedFilter,
    per_page: usize,
    nsfw: bool,
}

impl<'a> SearchBuilder<'a> {
    pub fn new() -> SearchBuilder<'a> {
        SearchBuilder {
            mod_type: TypeFilter::Mod,
            search: SearchFilter::Game { game_id: 11534 },
            feed: FeedFilter::Featured,
            per_page: 30,
            nsfw: false,
        }
    }

    pub fn of_type(mut self, mod_type: TypeFilter) -> Self {
        self.mod_type = mod_type;
        self
    }

    pub fn by_search(mut self, search: SearchFilter<'a>) -> Self {
        self.search = search;
        self
    }

    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;
        self
    }

    pub fn with_sort(mut self, sort: FeedFilter) -> Self {
        self.feed = sort;
        self
    }

    pub fn per_page(mut self, size: usize) -> Self {
        self.per_page = size;
        self
    }

    pub fn build(self) -> Search {
        let mut part = String::new();
        let per_page = self.per_page;
        match self.mod_type {
            TypeFilter::Mod => part.push_str("Mod/"),
            TypeFilter::Sound => part.push_str("Sound/"),
            TypeFilter::WiP => part.push_str("Wip/"),
        }
        match self.search {
            SearchFilter::Category { cat_id } => {
                part.push_str(&format!("ByCategory?_aCategoryRowIds[]={cat_id}"))
            }
            SearchFilter::Name { search, game_id } => {
                part.push_str(&format!("ByName?_sName=*{search}*&_idGameRow={game_id}&"))
            }
            SearchFilter::Game { game_id } => {
                part.push_str(&format!("ByGame?_aGameRowIds[]={game_id}&"))
            }
        }
        part.push_str(&format!(
            "_csvProperties=_sName,_sModelName,_idRow,\
            _aSubmitter,_tsDateUpdated,_tsDateAdded,_aPreviewMedia,_sText,\
            _sDescription,_aCategory,_aRootCategory,_aGame,_nViewCount,_nLikeCount,\
            _nDownloadCount,_bIsNsfw,_aAlternateFileSources&_nPerpage={per_page}",
        ));
        if !self.nsfw {
            part.push_str("&_aArgs[]=_sbIsNsfw = false");
        }
        match self.feed {
            FeedFilter::Popular => part.push_str("&_sOrderBy=_nDownloadCount,DESC"),
            FeedFilter::Featured => {
                part.push_str("&_aArgs[]=_sbWasFeatured = true& _sOrderBy=_tsDateAdded,DESC")
            }
            FeedFilter::Recent => part.push_str("&_sOrderBy=_tsDateUpdated,DESC"),
        }
        Search::base(&part)
    }
}

impl<'a> Default for SearchBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
