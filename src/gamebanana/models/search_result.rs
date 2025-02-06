use std::path::PathBuf;

use ratatui::{
    style::{Color, Stylize},
    widgets::Row,
};
use serde::{Deserialize, Serialize};

use super::{category::GBModCategory, game::GBGame, modpage::GBModPage, preview::GBPreviewMedia};
use anyhow::Result;

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
    pub async fn mod_page(&self) -> Result<GBModPage> {
        GBModPage::build(self.row)
            .await
            .map(|page| if self.is_nsfw { page.set_nsfw() } else { page })
    }

    pub async fn download_media(&self) -> Vec<PathBuf> {
        let mut collected_media = Vec::new();
        for media in self.preview_media.clone() {
            let m = media.fetch().await;
            collected_media.push(m);
        }
        collected_media.into_iter().filter_map(Result::ok).collect()
    }
}

impl From<GBSearchEntry> for Row<'_> {
    fn from(value: GBSearchEntry) -> Self {
        let row = Row::new(vec![
            value.name,
            value.category.name,
            value.view_count.to_string(),
            value.like_count.to_string(),
            value.download_count.to_string(),
            value.description,
        ]);
        if value.is_nsfw {
            row.bg(Color::LightRed)
        } else {
            row
        }
    }
}
