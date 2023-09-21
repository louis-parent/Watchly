use std::fs;
use toml;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use super::Watchly;

const CONFIG_FILE: &str = "configuration.toml";

#[derive(Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct Configuration {
    pub client: ClientConfiguration,
    pub hourly_rate: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct ClientConfiguration {
    pub name: Option<String>,
    pub address: Option<String>
}

impl Configuration {
    pub fn load() -> Self {
        fs::read_to_string(Self::configuration_path()).map_or_else(|_error| {
            Configuration::default()
        }, |config_string| {
            toml::from_str(&config_string).unwrap_or_else(|_error| {
                Configuration::default()
            })
        })
    }

    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(Self::configuration_path(), toml::to_string(self)?)?;

        Ok(())
    }

    fn configuration_path() -> PathBuf {
        Watchly::storage_path().join(CONFIG_FILE)
    }
}
