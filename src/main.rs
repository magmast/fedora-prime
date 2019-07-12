extern crate serde;
extern crate structopt;
extern crate toml;

mod config;
mod errors;
mod mode;
mod options;

use std::fs;
use std::io;
use std::path::Path;
use std::process;

use structopt::StructOpt;

use crate::config::Config;
use crate::errors::*;
use crate::mode::Mode;
use crate::options::Options;

const XORG_CONFIG_PATH: &str = "/etc/X11/xorg.conf.d/10-fedora-prime.conf";
const MODPROBE_CONFIG_PATH: &str = "/etc/modprobe.d/fedora-prime.conf";
const MODPROBE_CONFIG: &str = include_str!("../assets/modprobe.conf");
const MODULES_LOAD_CONFIG_PATH: &str = "/etc/modules-load.d/fedora-prime.conf";
const MODULES_LOAD_CONFIG: &str = include_str!("../assets/modules-load.conf");
const DISABLE_GPU_SCRIPT: &str = include_str!("../assets/disable-gpu.sh");
const MODE_VAR_PATH: &str = "/var/lib/fedora-prime.conf";
const CONFIG_PATH: &str = "/etc/fedora-prime.toml";

fn main() {
    match run() {
        Err(err) => eprintln!("{}", err),
        _ => {}
    }
}

fn run() -> Result<(), RunError> {
    let options = Options::from_args();
    let config = Config::read_or_default(CONFIG_PATH)?;

    match options.command {
        options::Command::Switch { mode, .. } => 
            switch(mode, config).map_err(|err| err.into()),

        options::Command::PrintMode =>
            print_mode().map_err(|err| err.into()),

        options::Command::DisableGpu =>
            disable_gpu().map_err(|err| err.into()),
    }
}

fn switch(mode: Mode, config: Config) -> Result<(), SwitchError> {
    match mode {
        Mode::Intel => {
            fs::write(MODPROBE_CONFIG_PATH, MODPROBE_CONFIG)?;

            let xorg_config = format!(
                include_str!("../assets/xorg.conf"),
                bus_id = config.intel().bus_id(),
                accel_method = config.intel().accel_method(),
                dri = config.intel().dri(),
                driver = config.intel().driver(),
            );

            fs::write(XORG_CONFIG_PATH, xorg_config)?;
            
            fs::write(MODULES_LOAD_CONFIG_PATH, MODULES_LOAD_CONFIG)?;
        }

        Mode::Nvidia => {
            remove_if_exists(MODPROBE_CONFIG_PATH)?;
            remove_if_exists(MODPROBE_CONFIG_PATH)?;
            remove_if_exists(MODULES_LOAD_CONFIG_PATH)?;
        }
    }

    fs::write(MODE_VAR_PATH, mode.to_string()).map_err(|err| err.into())
}

fn print_mode() -> Result<(), PrintModeError> {
    let mode = fs::read_to_string(MODE_VAR_PATH)
        .or_else(|err| {
            if err.kind() == io::ErrorKind::NotFound { 
                Ok("unset".into()) 
            } else { 
                Err(PrintModeError(err)) 
            }
        })?;

    println!("{}", mode);

    Ok(())
}

// TODO: Bad solution, think of a better one
fn disable_gpu() -> Result<(), DisableGpuError> {
    fs::write("disable-gpu.sh", DISABLE_GPU_SCRIPT)?;
    process::Command::new("sh").arg("disable-gpu.sh").spawn()?;
    remove_if_exists("disable-gpu.sh").map_err(|err| err.into())
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