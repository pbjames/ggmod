use crate::{gamebanana::models::modpage::GBModPage, ggst_path, registry};

use log::{info, trace};
use ordermap::OrderMap;
use serde::{Deserialize, Serialize};
use std::{fs, path};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type MutModClosure = dyn FnMut(&mut Mod) -> Result<()>;

#[derive(Debug, Clone)]
pub struct LocalCollection {
    registry_path: path::PathBuf,
    mods: Vec<Mod>,
}

impl Default for LocalCollection {
    fn default() -> Self {
        Self::new()
    }
}

/// Use this for managing mods locally stored
impl LocalCollection {
    pub fn new() -> LocalCollection {
        let path = registry().unwrap_or_default();
        trace!("New collection instance");
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
        Some(serde_json::from_reader(file).unwrap())
    }

    pub fn contains(&self, id: usize) -> Option<&Mod> {
        self.mods.iter().find(|m| m.id == id)
    }

    pub fn register_online_mod(&mut self, gbmod: GBModPage, idx: usize) -> Result<()> {
        let new_mod = Mod::build(gbmod, idx)?;
        self.mods.push(new_mod);
        Ok(())
    }

    pub fn apply_on_mod(&mut self, id: usize, mut closure: Box<MutModClosure>) -> Result<()> {
        for m in &mut self.mods {
            if m.id == id {
                closure(m)?;
            }
        }
        Ok(())
    }

    pub fn unstaged_mods(&self) -> OrderMap<String, usize> {
        trace!("Looking for unstaged mods, {:?}", self.mods);
        self.mods()
            .iter()
            .filter(|m| !m.staged)
            .map(|m| (m.name.clone(), m.id))
            .collect()
    }

    pub fn staged_mods(&self) -> OrderMap<String, usize> {
        self.mods()
            .iter()
            .filter(|m| m.staged)
            .map(|m| (m.name.clone(), m.id))
            .collect()
    }

    pub fn toggle(&mut self, id: usize) -> Result<()> {
        self.apply_on_mod(
            id,
            Box::new(|m: &mut Mod| if m.staged { m.unstage() } else { m.stage() }),
        )
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
    pub is_nsfw: bool,
}

impl Mod {
    fn build(gbmod: GBModPage, idx: usize) -> Result<Mod> {
        let m = Mod {
            id: gbmod.row,
            character: gbmod.category.name.clone(),
            path: gbmod.download_file(idx)?,
            name: gbmod.name,
            description: gbmod.description,
            staged: false,
            is_nsfw: gbmod.is_nsfw,
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
