use std::{fs, io, path};

use compress_tools::{uncompress_archive, Ownership};
use log::{debug, info, trace};
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use crate::download_path;

#[derive(Serialize, Deserialize, Debug)]
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
