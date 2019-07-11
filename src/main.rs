extern crate structopt;

use std::error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::io;
use std::path::Path;
use std::process::{self, Command};
use std::str::FromStr;

use structopt::StructOpt;

const XORG_CONFIG_PATH: &str = "/etc/X11/xorg.conf.d/10-fedora-prime.conf";
const XORG_CONFIG_INTEL: &str = include_str!("../assets/xorg.conf");

const MODPROBE_CONFIG_PATH: &str = "/etc/modprobe.d/fedora-prime.conf";
const MODPROBE_CONFIG: &str = include_str!("../assets/modprobe.conf");

const MODULES_LOAD_CONFIG_PATH: &str = "/etc/modules-load.d/fedora-prime.conf";
const MODULES_LOAD_CONFIG: &str = include_str!("../assets/modules-load.conf");

const DISABLE_GPU_SCRIPT: &str = include_str!("../assets/disable-gpu.sh");

const MODE_VAR_PATH: &str = "/var/lib/fedora-prime.conf";

fn main() {
    let opts = Options::from_args();
    match opts.command {
        Subcommand::Switch { mode, .. } => {
            if let Err(err) = switch(mode) {
                eprintln!("can't switch mode: {}", err);
                process::exit(1);
            }
        }

        Subcommand::PrintMode => {
            let mode = fs::read_to_string(MODE_VAR_PATH).unwrap_or_default();
            println!("{}", mode);
        }

        Subcommand::DisableGpu => {
            if let Err(err) = fs::write("disable-gpu.sh", DISABLE_GPU_SCRIPT) {
                eprintln!("can't create script: {}", err);
                process::exit(1);
            }

            if let Err(err) = Command::new("sh").arg("disable-gpu.sh").spawn() {
                eprintln!("can't run script: {}", err);
                process::exit(1);
            }

            if let Err(err) = remove_if_exists("disable-gpu.sh") {
                eprintln!("can't remove script: {}", err);
                process::exit(1);
            }
        }
    }
}

fn switch(mode: Mode) -> io::Result<()> {
    match mode {
        Mode::Intel => {
            fs::write(MODPROBE_CONFIG_PATH, MODPROBE_CONFIG)?;
            fs::write(XORG_CONFIG_PATH, XORG_CONFIG_INTEL)?;
            fs::write(MODULES_LOAD_CONFIG_PATH, MODULES_LOAD_CONFIG)?;
        }

        Mode::Nvidia => {
            remove_if_exists(MODPROBE_CONFIG_PATH)?;
            remove_if_exists(MODPROBE_CONFIG_PATH)?;
            remove_if_exists(MODULES_LOAD_CONFIG_PATH)?;
        }
    }

    fs::write(MODE_VAR_PATH, mode.to_string())
}

fn remove_if_exists<P: AsRef<Path>>(path: P) -> io::Result<()> {
    if let Err(err) = fs::remove_file(path) {
        if err.kind() != io::ErrorKind::NotFound {
            Err(err)
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "fedora-prime")]
struct Options {
    #[structopt(subcommand)]
    command: Subcommand,
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    #[structopt(name = "switch")]
    Switch {
        #[structopt()]
        mode: Mode,
    },

    #[structopt(name = "print-mode")]
    PrintMode,

    #[structopt(name = "disable-gpu")]
    DisableGpu,
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

#[derive(Debug, Copy, Clone)]
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