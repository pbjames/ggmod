use directories::BaseDirs;
use log::trace;
use std::{fs, io, path};

pub const SUBDIR_NAME: &str = "ggmod";
pub const REGISTRY_FN: &str = "registry.json";

pub fn check_download_path() -> std::result::Result<path::PathBuf, io::Error> {
    match BaseDirs::new() {
        Some(base) => {
            let path = base.cache_dir().join(SUBDIR_NAME).join("downloads");
            fs::DirBuilder::new().recursive(true).create(&path)?;
            Ok(path)
        }
        None => Err(io::Error::new(io::ErrorKind::NotFound, "Path inaccessible")),
    }
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
        trace!("Found path {:?} for steam root", path);
        Some(path)
    } else {
        None
    }
}

pub fn check_registry() -> Result<path::PathBuf, io::Error> {
    let reg_path = check_download_path()?.join(REGISTRY_FN);
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

    #[test]
    fn check_download_path_creates() {
        fn lift() -> Result<(), Box<dyn std::error::Error>> {
            let path = check_download_path()?;
            fs::remove_dir(&path)?;
            check_download_path()?;
            fs::read_dir(&path)?;
            Ok(())
        }
        lift().unwrap();
    }

    #[test]
    fn check_registry_creates() {
        let path = check_registry().unwrap();
        fs::remove_file(&path).unwrap();
        assert!(!path.exists() && !path.is_file());
        check_registry().unwrap();
        assert!(path.exists() && path.is_file());
    }
}
