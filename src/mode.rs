use std::error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    ModeNotExists,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::ModeNotExists => write!(f, "mode not exists"),
        }
    }
}

impl error::Error for Error {}

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Intel,
    Nvidia,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Mode::Intel => write!(f, "intel"),
            Mode::Nvidia => write!(f, "nvidia"),
        }
    }
}

impl FromStr for Mode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "intel" => Ok(Mode::Intel),
            "nvidia" => Ok(Mode::Nvidia),
            _ => Err(Error::ModeNotExists),
        }
    }
}