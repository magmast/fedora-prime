use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;

use toml::de;

#[derive(Debug)]
pub enum RunError {
    Config(ConfigError),
    Switch(SwitchError),
    PrintMode(PrintModeError),
    DisableGpu(DisableGpuError),
}

impl Display for RunError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use RunError::*;

        match self {
            Config(err) => write!(f, "Config error: {}", err),
            Switch(err) => write!(f, "Switch error: {}", err),
            PrintMode(err) => write!(f, "Print error: {}", err),
            DisableGpu(err) => write!(f, "Disable gpu error: {}", err),
        }
    }
}

impl From<ConfigError> for RunError {
    fn from(err: ConfigError) -> Self {
        RunError::Config(err)
    }
}

impl From<SwitchError> for RunError {
    fn from(err: SwitchError) -> Self {
        RunError::Switch(err)
    }
}

impl From<PrintModeError> for RunError {
    fn from(err: PrintModeError) -> Self {
        RunError::PrintMode(err)
    }
}

impl From<DisableGpuError> for RunError {
    fn from(err: DisableGpuError) -> Self {
        RunError::DisableGpu(err)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Read(io::Error),
    Parse(de::Error),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use ConfigError::*;

        match self {
            Read(err) => write!(f, "Read error: {}", err),
            Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::Read(err)
    }
}

impl From<de::Error> for ConfigError {
    fn from(err: de::Error) -> Self {
        ConfigError::Parse(err)
    }
}

#[derive(Debug)]
pub struct SwitchError(io::Error);

impl Display for SwitchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<io::Error> for SwitchError {
    fn from(err: io::Error) -> Self {
        Self(err)
    }
}

#[derive(Debug)]
pub struct PrintModeError(pub io::Error);

impl Display for PrintModeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<io::Error> for PrintModeError {
    fn from(err: io::Error) -> Self {
        Self(err)
    }
}

#[derive(Debug)]
pub struct DisableGpuError(pub io::Error);

impl Display for DisableGpuError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<io::Error> for DisableGpuError {
    fn from(err: io::Error) -> Self {
        Self(err)
    }
}