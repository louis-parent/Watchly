pub mod configuration;

use std::fs;
use std::error::Error;
use std::path::PathBuf;
use configuration::Configuration;

const STORAGE_DIR: &str = ".watchly";

pub struct Watchly {

}

impl Watchly {
    pub fn initialize(client_name: Option<String>, client_address: Option<String>, hourly_rate: Option<f64>) -> Result<(), Box<dyn Error>> {
        fs::create_dir(Watchly::storage_path())?;

        let mut configuration = Configuration::load();

        configuration.client.name = client_name;
        configuration.client.address = client_address;
        configuration.hourly_rate = hourly_rate;

        configuration.save()?;

        Ok(())
    }

    fn storage_path() -> PathBuf {
        PathBuf::from(STORAGE_DIR)
    }
}
