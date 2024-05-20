mod cfg;
mod cli;
mod params;

use crate::cfg::Cfg;
use crate::cli::Cli;
use crate::params::Params;
use clap::{crate_name, Parser};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg: Cfg = confy::load(crate_name!(), crate_name!())?;
    let params = Params::new(cli, cfg);

    println!("{:?}", params);

    Ok(())
}
