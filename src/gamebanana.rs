use std::{
    fs,
    io::{self, Cursor},
    path,
};

use compress_tools::{uncompress_archive, Ownership};
use log::info;
use reqwest::blocking;
use serde::{Deserialize, Serialize};

use crate::{check_gg_path, download_path, register_mod, registry_has_id};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// TODO: Refactor structs and json format so its less ugly
// inconsistent endpoint alternative:
// https://api.gamebanana.com/Core/Item/Data?itemtype=Mod&itemid={mod_id}&fields=Category().name,creator,date,description,downloads,Files().aFiles(),likes,name,Nsfw().bIsNsfw()&return_keys=true&format=json
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
    pub md5: String,
    pub file: String,
    pub download_url: String,
    pub description: String,
}

impl GBFile {
    pub fn download_to<'a>(&self, path: &'a path::PathBuf) -> Result<&'a path::PathBuf> {
        let response = blocking::get(&self.download_url)?;
        let mut file = fs::File::create(path)?;
        let mut content = Cursor::new(response.bytes()?);
        io::copy(&mut content, &mut file)?;
        Ok(path)
    }

    pub fn fetch(&self) -> Result<path::PathBuf> {
        let path = download_path().unwrap_or_default().join(&self.file);
        let mut dir = path.clone();
        dir.set_extension("");
        if dir.exists() && dir.is_dir() {
            Ok(dir)
        } else {
            self.download_to(&path)?;
            let src = fs::File::open(&path)?;
            uncompress_archive(src, &dir, Ownership::Preserve)?;
            info!(
                "{}",
                format!("Archive {:?} decompressed to {:?}", path, dir)
            );
            Ok(dir)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GBMod {
    pub category: GBCategory,
    pub files: Vec<GBFile>,
    pub name: String,
    pub description: String,
}

impl GBMod {
    pub fn files(&self) -> &Vec<GBFile> {
        &self.files
    }
    pub fn download_file(&self, idx: usize) -> Result<path::PathBuf> {
        self.files[idx].fetch()
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
        Ok(serde_json::from_str::<GBMod>(
            &resp
                .replace("_aCategory", "category")
                .replace("_aFiles", "files")
                .replace("_aFile", "file")
                .replace("_sFiles", "files")
                .replace("_sFile", "file")
                .replace("_sName", "name")
                .replace("_sDescription", "description")
                .replace("_bContainsExe", "contains_exe")
                .replace("_nDownloadCount", "download_count")
                .replace("_nFilesize", "filesize")
                .replace("_sAnalysisResultCode", "analysis_result_code")
                .replace("_tsDateAdded", "date_added")
                .replace("_sMd5Checksum", "md5")
                .replace("_sDownloadUrl", "download_url")
                .replace("_sIconUrl", "icon_url"),
        )?)
        //pub _bContainsExe: bool,
        //pub _nDownloadCount: usize,
        //pub _nFilesize: usize,
        //pub _sAnalysisResultCode: String,
        //pub _tsDateAdded: usize,
        //pub _sMd5Checksum: String,
        //pub file: String,
        //pub _sDownloadUrl: String,
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
            character: gbmod.category.name.clone(),
            path: gbmod.download_file(idx)?,
            name: gbmod.name,
            description: gbmod.description,
            staged: false,
        };
        register_mod(&m)?;
        Ok(m)
    }

    pub fn stage(&mut self) -> Result<()> {
        dircpy::copy_dir(
            &self.path,
            check_gg_path()
                .unwrap_or_default()
                .join(self.id.to_string()),
        )?;
        self.staged = true;
        Ok(())
    }

    pub fn unstage(&mut self) -> Result<()> {
        fs::remove_dir_all(
            check_gg_path()
                .unwrap_or_default()
                .join(self.id.to_string()),
        )?;
        self.staged = false;
        Ok(())
    }
}
