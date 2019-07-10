use std::env;
use std::error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::process;
use std::str::FromStr;

const XORG_CONFIG_PATH: &str = "/etc/X11/xorg.conf.d/10-fedora-prime.conf";
const XORG_CONFIG_INTEL: &str = include_str!("../assets/xorg.conf");

const MODPROBE_CONFIG_PATH: &str = "/etc/modprobe.d/fedora-prime.conf";
const MODPROBE_CONFIG: &str = include_str!("../assets/modprobe.conf");

const MODE_VAR_PATH: &str = "/var/lib/fedora-prime.conf";

fn main() {
    match env::args().nth(1) {
        None => print_mode(),

        Some(mode) => {
            match mode.parse() {
                Ok(mode) => set_mode(mode),

                Err(err) => {
                    eprintln!("{}", err);
                    process::exit(1);
                }
            }
        }
    };
}

fn print_mode() {
    let mode = match fs::read_to_string(MODE_VAR_PATH) {
        Ok(contents) => contents,

        Err(err) => {
            eprintln!("can't read mode: {}", err);
            process::exit(1);
        }
    };

    println!("{}", mode);
}

fn set_mode(mode: Mode) {
    match mode {
        Mode::Intel => {
            if let Err(err) = fs::write(MODPROBE_CONFIG_PATH, MODPROBE_CONFIG) {
                eprintln!("can't write modprobe config: {}", err);
                process::exit(1);
            }

            if let Err(err) = fs::write(XORG_CONFIG_PATH, XORG_CONFIG_INTEL) {
                eprintln!("can't write xorg config: {}", err);
                process::exit(1);
            }
        }

        Mode::Nvidia => {
            if let Err(err) = fs::remove_file(MODPROBE_CONFIG_PATH) {
                eprintln!("can't remove modprobe config: {}", err);
                process::exit(1);
            }

            if let Err(err) = fs::remove_file(MODPROBE_CONFIG_PATH) {
                eprintln!("can't remove xorg config: {}", err);
                process::exit(1);
            }
        }
    }

    if let Err(err) = fs::write(MODE_VAR_PATH, mode.to_string()) {
        eprintln!("can't write mode variable: {}", err);
        process::exit(1);
    }
}

#[derive(Debug)]
enum Error {
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

#[derive(Copy, Clone)]
enum Mode {
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