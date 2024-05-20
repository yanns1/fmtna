pub mod cfg;
pub mod cli;

use crate::cfg::Cfg;
use crate::cli::Cli;
use clap::{crate_name, Parser};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg: Cfg = confy::load(crate_name!(), crate_name!())?;

    println!("{:?}", cli);
    println!("{:?}", cfg);
    Ok(())
}
