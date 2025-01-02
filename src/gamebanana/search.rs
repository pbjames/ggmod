use log::{info, trace};

use crate::gamebanana::to_human;

use super::models::search_result::GBSearchEntry;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Search {
    url: String,
}

impl Search {
    // TODO: Could use some external cache later on
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
