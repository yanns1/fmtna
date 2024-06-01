mod cfg;
mod cli;
mod default;
mod engine;
mod exclude;
mod naming_conventions;
mod revert;

use crate::cfg::Cfg;
use crate::cli::Cli;
use crate::engine::get_engine;
use clap::{crate_name, Parser};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg: Cfg = confy::load(crate_name!(), crate_name!())?;

    let engine = get_engine(cli, cfg)?;
    engine.run()
}
