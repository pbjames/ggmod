use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Search {
    #[serde(skip)]
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

    pub fn read_page(&self, n: usize) {
        let url = self.page(n);
        info!("url generated: {url}");
        //let resp = reqwest::blocking::get(url)?;
        //let conv = Self::convert_to_snake(&resp);
        //Ok(serde_json::from_str::<GBSearch>(&conv)?)
    }
}
