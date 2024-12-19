use crate::gamebanana::GBModPage;
use crate::{check_gg_path, check_registry};

use log::info;
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
        let path = check_registry().unwrap_or_default();
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

    pub fn registry_has_id(&self, mod_id: usize) -> Result<Option<Mod>> {
        // TODO: Tiny problem, we need to stop fking cloning
        Ok(self.mods.iter().find(|m| m.id == mod_id).cloned())
    }

    pub fn register_online_mod(&mut self, gbmod: GBModPage, id: usize, idx: usize) -> Result<()> {
        // TODO: Could defer I/O to end of lifecycle
        let new_mod = Mod::build(gbmod, id, idx)?;
        let file = fs::OpenOptions::new()
            .write(true)
            .open(&self.registry_path)?;
        self.mods.push(new_mod);
        serde_json::to_writer(file, &self.mods)?;
        Ok(())
    }

    pub fn apply_on_mod(&self, id: usize, mut closure: Box<dyn FnMut(&mut Mod)>) {
        let mut chosen_mod = self
            .registry_has_id(id)
            .expect("Invalid ID or registry (delete it)")
            .expect("Couldn't find mod with that ID");
        closure(&mut chosen_mod);
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
            check_gg_path()
                .unwrap_or_default()
                .join(self.id.to_string()),
        )?;
        self.staged = true;
        Ok(())
    }

    // TODO: remove curse
    pub fn _staged(&self) -> bool {
        check_gg_path()
            .unwrap_or_default()
            .join(self.id.to_string())
            .is_dir()
    }

    pub fn unstage(&mut self) -> Result<()> {
        info!("Unstaging {}", self.name);
        fs::remove_dir_all(
            check_gg_path()
                .unwrap_or_default()
                .join(self.id.to_string()),
        )?;
        self.staged = false;
        Ok(())
    }
}
