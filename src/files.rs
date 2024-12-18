use directories::BaseDirs;
use log::info;
use serde::Serialize;
use std::{
    fs,
    io::{self, Read, Write},
};

use crate::gamebanana::Mod;

pub const SUBDIR_NAME: &str = "ggmod";
pub const REGISTRY_FN: &str = "registry.json";

pub fn download_path() -> Option<std::path::PathBuf> {
    Some(
        BaseDirs::new()?
            .cache_dir()
            .join(SUBDIR_NAME)
            .join("downloads"),
    )
}

pub fn check_download_path() -> Result<std::path::PathBuf, io::Error> {
    if let Some(path) = download_path() {
        fs::DirBuilder::new().recursive(true).create(&path)?;
        Ok(path)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Path inaccessible"))
    }
}

pub fn check_registry() -> Result<std::path::PathBuf, io::Error> {
    check_download_path()?;
    let reg_path = check_download_path()?.join(REGISTRY_FN);
    if !reg_path.is_file() {
        info!("Write new registry.json");
        fs::File::create(&reg_path)?;
    }
    Ok(reg_path)
}

pub fn registry_has_id(mod_id: usize) -> Result<Option<Mod>, io::Error> {
    // TODO: Use serde_json::from_reader and writer
    let path = check_registry()?;
    let mut file = fs::OpenOptions::new().read(true).open(path)?;
    let mut s_ptr = String::new();
    file.read_to_string(&mut s_ptr)?;
    let obj: Vec<Mod> = serde_json::from_str(&s_ptr)?;
    Ok(obj.iter().find(|m| m.id == mod_id).cloned())
}

pub fn register_object<T: Serialize>(obj: T) -> Result<(), io::Error> {
    // TODO: Use serde_json::from_reader and writer
    let path = check_registry()?;
    let mut file = fs::OpenOptions::new().append(true).open(path)?;
    writeln!(file, "{:?}", serde_json::to_string_pretty(&obj))
}
