use crate::files::*;

pub mod files;
pub mod gamebanana;

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    #[test]
    fn check_download_path_creates() {
        // TODO: This looks dumb fix it
        let path = download_path().unwrap();
        fs::remove_dir(&path).unwrap_or(());
        check_download_path().unwrap();
        fs::read_dir(&path).unwrap();
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
