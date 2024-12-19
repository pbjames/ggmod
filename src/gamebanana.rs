use std::{
    fs,
    io::{self, Cursor},
    path,
};

use reqwest::blocking;
use serde::{Deserialize, Serialize};

use crate::{check_gg_path, download_path, register_object, registry_has_id};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// TODO: Refactor structs and json format so its less ugly
// inconsistent endpoint alternative:
// https://api.gamebanana.com/Core/Item/Data?itemtype=Mod&itemid={mod_id}&fields=Category().name,creator,date,description,downloads,Files().aFiles(),likes,name,Nsfw().bIsNsfw()&return_keys=true&format=json
#[derive(Serialize, Deserialize, Debug)]
pub struct GBCategory {
    pub _sIconUrl: String,
    pub _sName: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GBFile {
    pub _bContainsExe: bool,
    pub _nDownloadCount: usize,
    pub _nFilesize: usize,
    pub _sAnalysisResultCode: String,
    pub _tsDateAdded: usize,
    pub _sMd5Checksum: String,
    pub _sFile: String,
    pub _sDownloadUrl: String,
    pub _sDescription: String,
}

impl GBFile {
    pub fn download_to(&self, path: path::PathBuf) -> Result<path::PathBuf> {
        let response = blocking::get(&self._sDownloadUrl)?;
        let mut file = fs::File::create(&path)?;
        let mut content = Cursor::new(response.bytes()?);
        io::copy(&mut content, &mut file)?;
        Ok(path)
    }

    pub fn fetch(&self) -> Result<path::PathBuf> {
        let path = download_path().unwrap_or_default().join(&self._sFile);
        if path.exists() && path.is_file() {
            Ok(path)
        } else {
            self.download_to(path)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GBMod {
    pub _aCategory: GBCategory,
    pub _aFiles: Vec<GBFile>,
    pub _sName: String,
    pub _sDescription: String,
}

impl GBMod {
    pub fn files(&self) -> &Vec<GBFile> {
        &self._aFiles
    }
    pub fn download_file(&self, idx: usize) -> Result<path::PathBuf> {
        self._aFiles[idx].fetch()
    }

    pub fn build(id: usize) -> Result<GBMod> {
        let uri = format!(
            "https://gamebanana.com/apiv6/Mod/{}?\
        _csvProperties=_sName,_aGame,_sProfileUrl,_aPreviewMedia,\
        _sDescription,_aSubmitter,_aCategory,_aSuperCategory,_aFiles,\
        _tsDateUpdated,_aAlternateFileSources,_bHasUpdates,_aLatestUpdates",
            id
        );
        let resp = blocking::get(uri)?.text()?;
        Ok(serde_json::from_str::<GBMod>(&resp)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mod {
    pub id: usize,
    pub character: String,
    path: path::PathBuf,
    pub name: String,
    pub description: String,
    pub staged: bool,
}

impl Mod {
    pub fn get(id: usize) -> Option<Mod> {
        registry_has_id(id).ok()?
    }

    pub fn build(id: usize, gbmod: GBMod, idx: usize) -> Result<Mod> {
        let m = Mod {
            id,
            character: gbmod._aCategory._sName.clone(),
            path: gbmod.download_file(idx)?,
            name: gbmod._sName,
            description: gbmod._sDescription,
            staged: false,
        };
        register_object(&m)?;
        Ok(m)
    }

    pub fn stage(&self) -> Result<()> {
        todo!()
    }
}
