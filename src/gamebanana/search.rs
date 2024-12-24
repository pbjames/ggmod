use log::{info, trace};
use serde::{Deserialize, Serialize};

use crate::gamebanana::to_human;

use super::modpage::GBCategory;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// TODO: Need models folder with models
#[derive(Serialize, Deserialize, Debug)]
pub struct GBPreviewMedia {
    pub base_url: String,
    pub file: String,
}

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

pub struct Search {
    url: String,
}

impl Search {
    pub fn base(s: &str) -> Search {
        Search {
            url: String::from("https://gamebanana.com/apiv6/") + s,
        }
    }

    fn page(&self, n: usize) -> String {
        self.url.clone() + &format!("&_nPage={n}")
    }

    pub fn read_page(&self, n: usize) -> Result<Vec<GBSearchEntry>> {
        let url = self.page(n);
        info!("url generated: {url}");
        let resp = reqwest::blocking::get(url)?.text()?;
        trace!("resp: {resp}");
        let conv = to_human(&resp)?;
        trace!("conv: {conv}");
        info!("successful search page conversion");
        Ok(serde_json::from_str::<Vec<GBSearchEntry>>(&conv)?)
    }
}
