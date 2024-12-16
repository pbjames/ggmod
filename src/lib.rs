use directories::BaseDirs;
use log::info;
use std::{fs, io};

pub const SUBDIR_NAME: &str = "ggmod";

fn download_path() -> Option<std::path::PathBuf> {
    Some(
        BaseDirs::new()?
            .cache_dir()
            .join(SUBDIR_NAME)
            .join("downloads"),
    )
}

pub fn check_download_path() -> Result<(), std::io::Error> {
    if let Some(path) = download_path() {
        info!("Download path not found, creating");
        fs::DirBuilder::new().recursive(true).create(path)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Path inaccessible"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_download_path_creates() {
        // TODO: This looks dumb fix it
        let path = download_path().unwrap();
        fs::remove_dir(&path).unwrap_or(());
        check_download_path().unwrap();
        fs::read_dir(&path).unwrap();
    }
}
