use std::{fs, io, path};

use compress_tools::{uncompress_archive, Ownership};
use log::{debug, info, trace};
use serde::{Deserialize, Serialize};

use crate::download_path;

use crate::gamebanana::to_human;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct GBCategory {
    pub icon_url: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GBFile {
    pub contains_exe: bool,
    pub download_count: usize,
    pub filesize: usize,
    pub analysis_result_code: String,
    pub date_added: usize,
    pub md5_checksum: String,
    pub file: String,
    pub download_url: String,
    pub description: String,
}

/// Use this to download mods, inspect them and add them to a local collection
#[derive(Serialize, Deserialize, Debug)]
pub struct GBModPage {
    pub category: GBCategory,
    pub files: Vec<GBFile>,
    pub name: String,
    pub description: String,
    pub row: usize,
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

impl GBModPage {
    pub fn download_file(&self, idx: usize) -> Result<path::PathBuf> {
        self.files[idx].fetch()
    }

    pub fn build(id: usize) -> Result<GBModPage> {
        let resp = reqwest::blocking::get(Self::fetch_uri(id))?.text()?;
        let conv = to_human(&resp)?;
        info!("successful mod page conversion");
        Ok(serde_json::from_str::<GBModPage>(&conv)?)
    }

    fn fetch_uri(id: usize) -> String {
        // INFO: Just in case they ever fix the key names
        // https://api.gamebanana.com/Core/Item/Data?
        // itemtype=Mod&itemid={mod_id}
        // &fields=Category().name,creator,date,description,downloads,
        // Files().aFiles(),likes,name,Nsfw().bIsNsfw()
        // &return_keys=true&format=json
        format!(
            "https://gamebanana.com/apiv6/Mod/{id}?\
        _csvProperties=_sName,_aGame,_sProfileUrl,_aPreviewMedia,\
        _sDescription,_aSubmitter,_aCategory,_aSuperCategory,_aFiles,\
        _tsDateUpdated,_aAlternateFileSources,_bHasUpdates,_aLatestUpdates,\
        _idRow",
        )
    }
}
