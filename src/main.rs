pub mod cli;

use crate::cli::Cli;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("{:?}", cli);
    Ok(())
}
