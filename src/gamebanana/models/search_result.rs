use serde::{Deserialize, Serialize};

use super::{category::GBModCategory, game::GBGame, modpage::GBModPage, preview::GBPreviewMedia};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBSearchEntry {
    pub date_updated: usize,
    pub date_added: usize,
    pub model_name: String,
    pub name: String,
    pub is_nsfw: bool,
    pub row: usize,
    pub preview_media: Vec<GBPreviewMedia>,
    pub download_count: usize,
    pub view_count: usize,
    pub like_count: usize,
    pub text: String,
    pub description: String,
    pub category: GBModCategory,
    pub game: GBGame,
}

impl GBSearchEntry {
    pub fn mod_page(&self) -> Result<GBModPage> {
        GBModPage::build(self.row).map(|page| page.set_nsfw())
    }
}
