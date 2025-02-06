use log::info;
use ratatui::widgets::Row;
use serde::{Deserialize, Serialize};

use crate::gamebanana::to_human;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBModCategory {
    pub row: usize,
    pub icon_url: String,
    pub name: String,
    //pub item_count: usize,
}

impl GBModCategory {
    pub async fn build(id: usize) -> Result<Vec<GBModCategory>> {
        let resp = reqwest::get(Self::url(id)).await?.text().await?;
        let conv = to_human(&resp)?;
        info!("successful mod page conversion");
        let mut cats = serde_json::from_str::<Vec<GBModCategory>>(&conv)?;
        cats.insert(
            0,
            GBModCategory {
                row: 0,
                icon_url: String::from(""),
                name: String::from("None"),
            },
        );
        Ok(cats)
    }

    fn url(id: usize) -> String {
        format!("https://gamebanana.com/apiv11/Mod/Categories?_idCategoryRow={id}&_sSort=a_to_z")
    }
}

impl From<GBModCategory> for Row<'_> {
    fn from(value: GBModCategory) -> Self {
        Row::new(vec![value.name])
    }
}
