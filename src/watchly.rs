pub mod initializer;

use std::path::PathBuf;
use home;

const STORAGE_DIR: &str = ".watchly";
const SETTINGS_FILE: &str = "settings.toml";

pub struct Watchly {

}

impl Watchly {
    fn local_storage_path() -> PathBuf {
        PathBuf::from(STORAGE_DIR)
    }

    fn global_storage_path() -> PathBuf {
        home::home_dir().expect("Cannot find home directory").join(STORAGE_DIR)
    }

    fn local_settings_path() -> PathBuf {
        Watchly::local_storage_path().join(SETTINGS_FILE)
    }

    fn global_settings_path() -> PathBuf {
        Watchly::global_storage_path().join(SETTINGS_FILE)
    }
}
