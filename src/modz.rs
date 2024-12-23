use crate::gamebanana::GBModPage;
use crate::{ggst_path, registry};

use log::{info, trace};
use serde::{Deserialize, Serialize};
use std::{fs, path};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct LocalCollection {
    registry_path: path::PathBuf,
    mods: Vec<Mod>,
}

impl Default for LocalCollection {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalCollection {
    pub fn new() -> LocalCollection {
        let path = registry().unwrap_or_default();
        LocalCollection {
            registry_path: path.clone(),
            mods: Self::load_mods(&path).unwrap_or_default(),
        }
    }

    pub fn mods(&self) -> &Vec<Mod> {
        &self.mods
    }

    fn load_mods(path: &path::PathBuf) -> Option<Vec<Mod>> {
        let file = fs::OpenOptions::new().read(true).open(path).unwrap();
        trace!("Reading {:?} into LocalCollection", path);
        Some(serde_json::from_reader(file).unwrap())
    }

    pub fn registry_has_id(&self, mod_id: usize) -> bool {
        self.mods.iter().any(|m| m.id == mod_id)
    }

    pub fn register_online_mod(&mut self, gbmod: GBModPage, idx: usize) -> Result<()> {
        let mod_id = gbmod.id;
        let new_mod = Mod::build(gbmod, mod_id, idx)?;
        self.mods.push(new_mod);
        Ok(())
    }

    pub fn apply_on_mod(&mut self, id: usize, mut closure: Box<dyn FnMut(&mut Mod)>) -> Result<()> {
        for m in &mut self.mods {
            if m.id == id {
                closure(m);
            }
        }
        Ok(())
    }

    fn write_mods(&self) -> Option<()> {
        let file = fs::OpenOptions::new()
            .write(true)
            .open(&self.registry_path)
            .ok()?;
        file.set_len(0).ok()?;
        trace!("Drop LocalCollection, write to {:?}", &self.registry_path);
        serde_json::to_writer(file, &self.mods).ok()
    }
}

impl Drop for LocalCollection {
    fn drop(&mut self) {
        self.write_mods();
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
    fn build(gbmod: GBModPage, id: usize, idx: usize) -> Result<Mod> {
        let m = Mod {
            id,
            character: gbmod.category.name.clone(),
            path: gbmod.download_file(idx)?,
            name: gbmod.name,
            description: gbmod.description,
            staged: false,
        };
        Ok(m)
    }

    pub fn stage(&mut self) -> Result<()> {
        info!("Staging {}", self.name);
        dircpy::copy_dir(
            &self.path,
            ggst_path().unwrap_or_default().join(self.id.to_string()),
        )?;
        self.staged = true;
        Ok(())
    }

    pub fn unstage(&mut self) -> Result<()> {
        info!("Unstaging {}", self.name);
        fs::remove_dir_all(ggst_path().unwrap_or_default().join(self.id.to_string()))?;
        self.staged = false;
        Ok(())
    }
}
