use log::info;
use serde::{Deserialize, Serialize};

use crate::gamebanana::to_human;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct GBModCategory {
    pub row: usize,
    pub icon_url: String,
    pub name: String,
    pub item_count: usize,
}

impl GBModCategory {
    // TODO: Add to UI and responsive
    pub fn build(id: usize) -> Result<Vec<GBModCategory>> {
        let resp = reqwest::blocking::get(Self::url(id))?.text()?;
        let conv = to_human(&resp)?;
        info!("successful mod page conversion");
        Ok(serde_json::from_str::<Vec<GBModCategory>>(&conv)?)
    }

    fn url(id: usize) -> String {
        format!("https://gamebanana.com/apiv11/Mod/Categories?_idCategoryRow={id}&_sSort=a_to_z")
    }
}
