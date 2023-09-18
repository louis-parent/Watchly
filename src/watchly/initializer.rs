use std::fs;
use std::error::Error;
use toml::{self, Table, Value};

use super::Watchly;

impl Watchly {
    pub fn initialize(client_name: Option<String>, client_address: Option<String>, hourly_rate: Option<f64>) -> Result<(), Box<dyn Error>> {
        fs::create_dir(Watchly::local_storage_path())?;

        let mut local_settings = Table::new();

        if client_name.is_some() {
            local_settings.insert("client_name".to_string(), Value::String(client_name.unwrap()));
        }

        if client_address.is_some() {
            local_settings.insert("client_address".to_string(), Value::String(client_address.unwrap()));
        }

        if hourly_rate.is_some() {
            local_settings.insert("hourly_rate".to_string(), Value::Float(hourly_rate.unwrap()));
        }

        fs::write(Watchly::local_settings_path(), toml::to_string_pretty(&local_settings)?)?;

        Ok(())
    }
}
