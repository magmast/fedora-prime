mod mode;

use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

use crate::mode::Mode;

const MODPROBE_CONFIG_PATH: &str = "/etc/modprobe.d/fedora-prime.conf";
const MODPROBE_CONFIG: &str = include_str!("../assets/modprobe.conf");
const MODE_VAR_PATH: &str = "/var/lib/fedora-prime/mode";

fn main() {
    let mode = env::args().nth(1);
    match mode {
        Some(mode) => {
            let result = match mode.as_str() {
                "intel" => switch(Mode::Intel),

                "nvidia" => switch(Mode::Nvidia),

                mode => {
                    eprintln!("Unknown mode {}", mode);
                    process::exit(1);
                }
            };

            if let Err(err) = result {
                eprintln!("{}", err);
                process::exit(1);
            }
        }

        None => {
            eprintln!("Mode not specified!");
            eprintln!("Usage:");
            eprintln!("\tfedora-prime [intel/nvidia]");
        }
    };
}

fn switch(mode: Mode) -> Result<(), io::Error> {
    match mode {
        Mode::Intel => {
            fs::write(MODPROBE_CONFIG_PATH, MODPROBE_CONFIG)?;
        }

        Mode::Nvidia => {
            remove_if_exists(MODPROBE_CONFIG_PATH)?;
        }
    }

    write_mode(mode)
}

fn write_mode(mode: Mode) -> Result<(), io::Error> {
    if let Err(err) = fs::write(MODE_VAR_PATH, mode.to_string()) {
        if err.kind() == io::ErrorKind::NotFound {
            let path = Path::new(MODE_VAR_PATH).parent().unwrap();
            match fs::create_dir_all(path) {
                Ok(_) => write_mode(mode),
                Err(err) => Err(err),
            }
        } else {
            Err(err)
        }
    } else {
        Ok(())
    }
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