use std::path;

use log::info;
use serde::{Deserialize, Serialize};

use crate::gamebanana::to_human;

use super::{category::GBModCategory, file::GBFile};
use anyhow::Result;

fn default_nsfw() -> bool {
    false
}

/// Use this to download mods, inspect them and add them to a local collection
#[derive(Serialize, Deserialize, Debug)]
pub struct GBModPage {
    pub category: GBModCategory,
    pub files: Vec<GBFile>,
    pub name: String,
    pub description: String,
    pub row: usize,
    #[serde(default = "default_nsfw")]
    pub is_nsfw: bool,
}

const PROPS: [&str; 14] = [
    "_sName",
    "_aGame",
    "_sProfileUrl",
    "_aPreviewMedia",
    "_sDescription",
    "_aSubmitter",
    "_aCategory",
    "_aSuperCategory",
    "_aFiles",
    "_tsDateUpdated",
    "_aAlternateFileSources",
    "_bHasUpdates",
    "_aLatestUpdates",
    "_idRow",
];

impl GBModPage {
    pub fn download_file(&self, idx: usize) -> Result<path::PathBuf> {
        self.files[idx].fetch()
    }

    pub fn set_nsfw(mut self) -> Self {
        self.is_nsfw = true;
        self
    }

    pub fn build(id: usize) -> Result<GBModPage> {
        let resp = reqwest::blocking::get(Self::url(id))?.text()?;
        let conv = to_human(&resp)?;
        info!("successful mod page conversion");
        Ok(serde_json::from_str::<GBModPage>(&conv)?)
    }

    fn url(id: usize) -> String {
        // INFO: Could switch to api.gamebanana.com in future
        format!("https://gamebanana.com/apiv6/Mod/{id}?_csvProperties=") + &PROPS.join(",")
    }
}
