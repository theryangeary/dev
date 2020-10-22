use crate::subcommand::*;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "dev",
    about = "developer utility for performing common functions"
)]
pub struct Opt {
    /// Subcommand to run
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    Promote(Promote),
}

impl Opt {
    pub fn run(self) -> Result<()> {
        match self.subcommand {
            Subcommand::Promote(p) => p.run(),
        }
    }
}
