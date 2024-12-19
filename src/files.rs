use directories::BaseDirs;
use log::{debug, info, trace, warn};
use serde::Serialize;
use std::{
    env::home_dir,
    fs,
    io::{self, Read, Write},
    path,
};

use crate::gamebanana::Mod;

pub const SUBDIR_NAME: &str = "ggmod";
pub const REGISTRY_FN: &str = "registry.json";

pub fn download_path() -> Option<path::PathBuf> {
    Some(
        BaseDirs::new()?
            .cache_dir()
            .join(SUBDIR_NAME)
            .join("downloads"),
    )
}

pub fn check_download_path() -> Result<path::PathBuf, io::Error> {
    if let Some(path) = download_path() {
        fs::DirBuilder::new().recursive(true).create(&path)?;
        Ok(path)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Path inaccessible"))
    }
}

pub fn check_registry() -> Result<path::PathBuf, io::Error> {
    let reg_path = check_download_path()?.join(REGISTRY_FN);
    if !reg_path.is_file() {
        info!("Write new registry.json");
        fs::File::create(&reg_path)?;
    }
    Ok(reg_path)
}

pub fn load_mods(path: &path::PathBuf) -> Option<Vec<Mod>> {
    let file = fs::OpenOptions::new().read(true).open(path).unwrap();
    Some(serde_json::from_reader(file).unwrap())
}

pub fn registry_has_id(mod_id: usize) -> Result<Option<Mod>, io::Error> {
    let path = check_registry()?;
    let obj: Vec<Mod> = load_mods(&path).unwrap_or_default();
    Ok(obj.iter().find(|m| m.id == mod_id).cloned())
}

pub fn register_mod(obj: &Mod) -> Result<(), io::Error> {
    let path = check_registry()?;
    let mut prev = load_mods(&path).unwrap_or_default();
    prev.append(&mut vec![obj.clone()]); // TODO: This wasteful
    let file = fs::OpenOptions::new().append(true).open(path)?;
    Ok(serde_json::to_writer(file, &prev)?)
}

pub fn check_gg_path() -> Option<path::PathBuf> {
    // TODO: This will probably need new entries
    let steamroot = [
        directories::UserDirs::new()?
            .home_dir()
            .join(".steam")
            .join("root"),
        path::PathBuf::from("C:\\Program Files (x86)\\Steam\\"),
    ]
    .into_iter()
    .reduce(|acc, path| if path.exists() { path } else { acc })?;
    if steamroot.exists() {
        let path = steamroot
            .join("steamapps")
            .join("common")
            .join("GUILTY GEAR STRIVE")
            .join("RED")
            .join("Content")
            .join("Paks")
            .join("~mods");
        fs::DirBuilder::new().recursive(true).create(&path).ok()?;
        info!("Found path {:?} for steam root", path);
        Some(path)
    } else {
        None
    }
}
