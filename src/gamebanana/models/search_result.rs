use serde::{Deserialize, Serialize};

use super::{category::GBCategory, preview::GBPreviewMedia};
#[derive(Serialize, Deserialize, Debug)]
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
    pub category: GBCategory,
    //game: GBGame
}
