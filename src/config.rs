use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::errors::ConfigError;

#[derive(Deserialize, Default)]
pub struct Config {
    intel: Option<Intel>,
}

impl Config {
    pub fn read_or_default<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
        let path: &Path = path.as_ref();

        if path.exists() {
            fs::read_to_string(path)
                .map_err(|err| err.into())
                .map(|contents| toml::from_str(&contents))
                .map(|config| config.map_err(|err| err.into()))
                .and_then(|config| config)
        } else {
            Ok(Default::default())
        }
    }
}

#[derive(Deserialize, Default)]
pub struct Intel {
    driver: Option<String>,
    bus_id: Option<String>,
    accel_method: Option<String>,
}
