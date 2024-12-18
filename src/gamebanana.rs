use reqwest::blocking;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GBCategory {
    _sIconUrl: String,
    _sName: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GBFile {
    _bContainsExe: bool,
    _nDownloadCount: usize,
    _nFilesize: usize,
    _sAnalysisResultCode: String,
    _tsDateAdded: usize,
    _sMd5Checksum: String,
    _sFile: String,
    _sDownloadUrl: String,
    _sDescription: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GBMod {
    _aCategory: GBCategory,
    _aFiles: Vec<GBFile>,
    _sName: String,
    _sDescription: String,
}

pub fn download(mod_id: usize) -> Option<GBMod> {
    // TODO: This needs to return more context if it dies
    let uri = format!(
        "https://gamebanana.com/apiv6/Mod/{}?\
        _csvProperties=_sName,_aGame,_sProfileUrl,_aPreviewMedia,\
        _sDescription,_aSubmitter,_aCategory,_aSuperCategory,_aFiles,\
        _tsDateUpdated,_aAlternateFileSources,_bHasUpdates,_aLatestUpdates",
        mod_id
    );
    let resp = blocking::get(uri).ok()?.text().ok()?;
    serde_json::from_str(&resp).ok()?
}
