use log::info;
use serde::{Deserialize, Serialize};

use crate::gamebanana::to_human;

use super::modpage::GBCategory;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// TODO: Need models folder with models
#[derive(Serialize, Deserialize, Debug)]
pub struct GBPreviewMedia {
    base_url: String,
    file: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GBSearchEntry {
    date_updated: usize,
    date_added: usize,
    model_name: String,
    name: String,
    is_nsfw: bool,
    row: usize,
    preview_media: Vec<GBPreviewMedia>,
    download_count: usize,
    view_count: usize,
    like_count: usize,
    text: String,
    description: String,
    category: GBCategory,
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
        println!("RAW: {resp}");
        let conv = to_human(&resp)?;
        println!("CON: {resp}");
        info!("successful search page conversion");
        Ok(serde_json::from_str::<Vec<GBSearchEntry>>(&conv)?)
    }
}
