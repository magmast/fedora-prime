use structopt::StructOpt;

use crate::mode::Mode;

#[derive(StructOpt, Debug)]
#[structopt(name = "fedora-prime")]
pub struct Options {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
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