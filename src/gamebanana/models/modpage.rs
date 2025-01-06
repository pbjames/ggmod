use std::path;

use log::info;
use serde::{Deserialize, Serialize};

use crate::gamebanana::to_human;

use super::{category::GBModCategory, file::GBFile};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Use this to download mods, inspect them and add them to a local collection
#[derive(Serialize, Deserialize, Debug)]
pub struct GBModPage {
    pub category: GBModCategory,
    pub files: Vec<GBFile>,
    pub name: String,
    pub description: String,
    pub row: usize,
    #[serde(default = "uhh")]
    pub is_nsfw: bool,
}

fn uhh() -> bool {
    false
}

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
        format!(
            "https://gamebanana.com/apiv6/Mod/{id}?\
        _csvProperties=_sName,_aGame,_sProfileUrl,_aPreviewMedia,\
        _sDescription,_aSubmitter,_aCategory,_aSuperCategory,_aFiles,\
        _tsDateUpdated,_aAlternateFileSources,_bHasUpdates,_aLatestUpdates,\
        _idRow",
        )
    }
}
