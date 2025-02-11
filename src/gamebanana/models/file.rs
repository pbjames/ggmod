use std::{fs, io, path};

use anyhow::Result;
use compress_tools::{uncompress_archive, Ownership};
use log::{debug, info, trace, warn};
use ratatui::widgets::Row;
use serde::{Deserialize, Serialize};
use unrar::Archive;

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
    async fn download_to<'a>(&self, path: &'a path::PathBuf) -> Result<&'a path::PathBuf> {
        info!("Downloading new archive..");
        let response = reqwest::get(&self.download_url).await?;
        let mut file = fs::File::create(path)?;
        let mut content = io::Cursor::new(response.bytes().await?);
        io::copy(&mut content, &mut file)?;
        Ok(path)
    }

    pub async fn fetch(&self) -> Result<path::PathBuf> {
        let file = download_path().unwrap_or_default().join(&self.file);
        let dir = file.with_extension("");
        if dir.exists() && dir.is_dir() {
            trace!("Mod already exists, doing nothing");
        } else {
            self.download_to(&file).await?;
            debug!("Archive {file:?} attempting decompress to {dir:?}");
            if let Some(ext) = file.extension() {
                if ext == "rar" {
                    let mut archive = Archive::new(&file).open_for_processing().unwrap();
                    while let Some(header) = archive.read_header()? {
                        archive = if header.entry().is_file() {
                            header.extract_with_base(&dir)?
                        } else {
                            header.skip()?
                        };
                    }
                } else {
                    let src = fs::File::open(&file)?;
                    uncompress_archive(src, &dir, Ownership::Preserve)?;
                }
            } else {
                warn!("Extentionless archive, wtf: {file:?}");
            }
        }
        Ok(dir)
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
