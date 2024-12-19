use std::{fs, path};

use crate::{check_download_path, check_gg_path, gamebanana::GBMod};

use log::info;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const REGISTRY_FN: &str = "registry.json";

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

pub fn load_mods(path: &path::PathBuf) -> Option<Vec<Mod>> {
    let file = fs::OpenOptions::new().read(true).open(path).unwrap();
    Some(serde_json::from_reader(file).unwrap())
}

pub fn registry_has_id(mod_id: usize) -> Result<Option<Mod>> {
    let path = check_registry()?;
    let obj: Vec<Mod> = load_mods(&path).unwrap_or_default();
    Ok(obj.iter().find(|m| m.id == mod_id).cloned())
}

pub fn register_mod(obj: &Mod) -> Result<()> {
    let path = check_registry()?;
    let mut prev = load_mods(&path).unwrap_or_default();
    prev.append(&mut vec![obj.clone()]); // TODO: This wasteful
    let file = fs::OpenOptions::new().write(true).open(path)?;
    Ok(serde_json::to_writer(file, &prev)?)
}

pub fn check_registry() -> Result<path::PathBuf> {
    let reg_path = check_download_path()?.join(REGISTRY_FN);
    if !reg_path.is_file() {
        info!("Write new registry.json");
        fs::File::create(&reg_path)?;
        let file = fs::OpenOptions::new().append(true).open(&reg_path)?;
        serde_json::to_writer(file, &Vec::<Mod>::new())?;
    }
    Ok(reg_path)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_registry_creates() {
        let path = check_registry().unwrap();
        fs::remove_file(&path).unwrap();
        assert!(!path.exists() && !path.is_file());
        check_registry().unwrap();
        assert!(path.exists() && path.is_file());
    }
}
