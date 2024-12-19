use directories::BaseDirs;
use log::info;
use std::{fs, io, path};

pub const SUBDIR_NAME: &str = "ggmod";

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
        info!("Found path {:?} for steam root", path);
        Some(path)
    } else {
        None
    }
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
}
