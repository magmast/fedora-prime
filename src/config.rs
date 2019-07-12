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

    pub fn intel(&self) -> Intel {
        self.intel.as_ref().map(|intel| intel.clone()).unwrap_or_default()
    }
}

#[derive(Deserialize, Default, Clone)]
pub struct Intel {
    driver: Option<String>,
    dri: Option<String>,
    bus_id: Option<String>,
    accel_method: Option<String>,
}

impl Intel {
    pub fn driver(&self) -> String {
        self.driver
            .as_ref()
            .map(|driver| driver.clone())
            .unwrap_or("modesetting".into())
    }

    pub fn dri(&self) -> String {
        self.dri
            .as_ref()
            .map(|dri| dri.clone())
            .unwrap_or("3".into())
    }

    pub fn bus_id(&self) -> String {
        self.bus_id
            .as_ref()
            .map(|bus_id| bus_id.clone())
            .unwrap_or("PCI:0:0:0".into())
    }

    pub fn accel_method(&self) -> String {
        self.accel_method
            .as_ref()
            .map(|accel_method| accel_method.clone())
            .unwrap_or("sna".into())
    }
}
