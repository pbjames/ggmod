use std::{fs, io, path};

use anyhow::Result;
use compress_tools::{uncompress_archive, Ownership};
use log::{debug, info, trace};
use ratatui::widgets::Row;
use serde::{Deserialize, Serialize};

use crate::download_path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBFile {
    pub contains_exe: bool,
    pub download_count: usize,
    pub filesize: usize,
    pub analysis_result_code: String,
    pub date_added: usize,
    pub file: String,
    pub download_url: String,
    pub description: String,
}

impl GBFile {
    pub fn download_to<'a>(&self, path: &'a path::PathBuf) -> Result<&'a path::PathBuf> {
        info!("Downloading new archive..");
        let response = reqwest::blocking::get(&self.download_url)?;
        let mut file = fs::File::create(path)?;
        let mut content = io::Cursor::new(response.bytes()?);
        io::copy(&mut content, &mut file)?;
        Ok(path)
    }

    pub fn fetch(&self) -> Result<path::PathBuf> {
        let file = download_path().unwrap_or_default().join(&self.file);
        let dir = file.with_extension("");
        if dir.exists() && dir.is_dir() {
            trace!("Mod already exists, doing nothing");
            Ok(dir)
        } else {
            self.download_to(&file)?;
            let src = fs::File::open(&file)?;
            uncompress_archive(src, &dir, Ownership::Preserve)?;
            debug!("{}", format!("Archive {file:?} decompressed to {dir:?}"));
            Ok(dir)
        }
    }
}

impl From<GBFile> for Row<'_> {
    fn from(value: GBFile) -> Self {
        Row::new(vec![
            value.file,
            value.download_count.to_string(),
            value.description,
        ])
    }
}
