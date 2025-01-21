use std::path::PathBuf;

use log::{debug, info, trace};
use serde::{Deserialize, Serialize};

use std::{
    fs,
    io::{self},
};

use anyhow::Result;

use crate::download_path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBPreviewMedia {
    #[serde(alias = "type")]
    pub media_type: String,
    pub base_url: String,
    pub file: String,
}

impl GBPreviewMedia {
    pub fn fetch(&self) -> Result<PathBuf> {
        let file = download_path().unwrap_or_default().join(&self.file);
        if file.exists() && file.is_file() {
            trace!("Preview media already exists, doing nothing");
        } else {
            debug!("{}", format!("Archive {file:?} attempting to download"));
            self.download_to(&file)?;
        }
        Ok(file)
    }

    fn download_to<'a>(&self, path: &'a PathBuf) -> Result<&'a PathBuf> {
        info!("Downloading new archive..");
        let url = format!("{}/{}", self.base_url.clone(), &self.file);
        let response = reqwest::blocking::get(url)?;
        let mut file = fs::File::create(path)?;
        let mut content = io::Cursor::new(response.bytes()?);
        io::copy(&mut content, &mut file)?;
        Ok(path)
    }
}
