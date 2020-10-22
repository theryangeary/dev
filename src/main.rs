use anyhow::Result;
use structopt::StructOpt;

mod opt;
mod subcommand;

use opt::Opt;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    opt.run()
}
