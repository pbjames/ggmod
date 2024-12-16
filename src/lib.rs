use directories::BaseDirs;
use http::{Request, Response};
use std::fs::{self, DirBuilder, File};

const SUBDIR_NAME: &str = "ggmod";

fn cache_path() -> Option<std::path::PathBuf> {
    Some(BaseDirs::new()?.config_dir().join(SUBDIR_NAME))
}

fn check_cache_path() -> Result<(), std::io::Error> {
    // TODO: Erase unwrap call
    DirBuilder::new()
        .recursive(true)
        .create(cache_path().unwrap())
}

//fn download_mod(mod_id: usize) -> Result<std::path::PathBuf, &str> {}
