use directories::{ProjectDirs, UserDirs};
use log::{trace, warn};
use std::{
    fs,
    io::{self, Result},
    path::{self},
};

pub const SUBDIR_NAME: &str = "ggmod";
pub const REGISTRY_FN: &str = "registry.json";

pub fn not_found(s: &str) -> io::Error {
    io::Error::new(io::ErrorKind::NotFound, s)
}

pub fn ggmod_root() -> Result<ProjectDirs> {
    match ProjectDirs::from("mod", "sigma", "ggmod") {
        Some(path) => Ok(path),
        None => Err(not_found("GGMod path inaccessible")),
    }
}

pub fn download_path() -> Result<path::PathBuf> {
    let proj_root = ggmod_root()?;
    let dl_path = proj_root.data_dir().join("downloads");
    fs::DirBuilder::new().recursive(true).create(&dl_path)?;
    Ok(dl_path)
}

pub fn steam_root() -> Result<path::PathBuf> {
    // TODO: This will probably need new entries + replace exists call
    let steamroot = [
        UserDirs::new()
            .ok_or(not_found("User dir path inaccessible"))?
            .home_dir()
            .join(".steam")
            .join("root"),
        path::PathBuf::from("C:\\Program Files (x86)\\Steam\\"),
        path::PathBuf::from("C:\\Program Files\\Steam\\"),
    ]
    .into_iter()
    .reduce(|acc, path| if path.exists() { path } else { acc })
    .filter(|p| p.exists());
    steamroot.ok_or(not_found("steam root inaccessible"))
}

pub fn ggst_path() -> Result<path::PathBuf> {
    let path = steam_root()?
        .join("steamapps")
        .join("common")
        .join("GUILTY GEAR STRIVE")
        .join("RED")
        .join("Content")
        .join("Paks")
        .join("~mods");
    fs::DirBuilder::new().recursive(true).create(&path)?;
    trace!("Found path {:?} for steam root", path);
    Ok(path)
}

pub fn game_sig_file() -> Result<path::PathBuf> {
    Ok(ggst_path()?
        .parent()
        .unwrap()
        .join("pakchunk0-WindowsNoEditor.sig"))
}

pub fn ensure_sig_file(path: &path::Path) -> Result<()> {
    let has_sig = path.read_dir()?.any(|entry| {
        entry.is_ok_and(|entry| entry.path().extension().is_some_and(|ext| ext == "sig"))
    });
    let name = path.read_dir()?.find(|entry| {
        entry
            .as_ref()
            .is_ok_and(|entry| entry.path().extension().is_some_and(|ext| ext == "pak"))
    });
    trace!(
        "Have sig in {path:?}: {has_sig}, have name: {}",
        name.is_some()
    );
    if !has_sig && name.is_some() {
        trace!(
            "Copy {:?} to {:?}",
            game_sig_file()?,
            name.as_ref().unwrap().as_ref().unwrap().path()
        );
        fs::copy(
            game_sig_file()?,
            name.unwrap().unwrap().path().with_extension("sig"),
        )?;
    } else if !has_sig && name.is_none() {
        warn!("Have no .sig in {path:?} but also no .pak");
    }
    Ok(())
}

pub fn registry() -> Result<path::PathBuf> {
    let reg_path = download_path()?.join(REGISTRY_FN);
    if !reg_path.is_file() {
        trace!("Write new registry.json");
        fs::File::create(&reg_path)?;
        let file = fs::OpenOptions::new().append(true).open(&reg_path)?;
        serde_json::to_writer(file, &Vec::<usize>::new())?;
    }
    Ok(reg_path)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{env, thread, time};

    #[test]
    fn download_path_creates() {
        let path = download_path().unwrap();
        thread::sleep(time::Duration::from_millis(20));
        fs::remove_dir_all(&path).unwrap();
        download_path().unwrap();
        fs::read_dir(&path).unwrap();
    }

    #[test]
    fn registry_creates() {
        let path = registry().unwrap();
        fs::remove_file(&path).unwrap();
        assert!(!path.exists() && !path.is_file());
        registry().unwrap();
        assert!(path.exists() && path.is_file());
    }

    #[test]
    fn ggmod_root_works() {
        ggmod_root().unwrap();
    }

    #[test]
    fn steam_root_finds() {
        // TODO: Currently macos users aren't real and can't cross test between os
        match env::consts::OS {
            "windows" => {
                let path = path::PathBuf::from("C:\\Program Files (x86)\\Steam\\");
                fs::DirBuilder::new().recursive(true).create(path).unwrap();
                steam_root().unwrap();
            }
            "linux" => {
                let path = UserDirs::new()
                    .unwrap()
                    .home_dir()
                    .join(".steam")
                    .join("root");
                fs::DirBuilder::new().recursive(true).create(path).unwrap();
                steam_root().unwrap();
            }
            "macos" => {
                todo!()
            }
            _ => {
                todo!()
            }
        }
    }
}
