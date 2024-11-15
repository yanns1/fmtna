//! Everything related to the app's CLI.

use crate::cfg::Cfg;
use crate::cli::Cli;
use crate::cli::Command;
use crate::default;
use crate::exclude;
use crate::revert;

/// A subcommand engine, a structure that encapsulates the logic of a subcommand.
pub trait Engine {
    /// Runs the engine.
    fn run(&mut self) -> anyhow::Result<()>;
}

/// Returns the engine corresponding to the given `cli` and `cfg`.
///
/// # Parameters
///
/// - `cli`: The CLI arguments.
/// - `cfg`: The configuration values.
///
/// # Returns
///
/// The parametrized engine for running the app's logic, or an error
/// if engine creation failed.
///
/// # Examples
///
/// ```rust,no_run
/// use clap::Parser;
/// use fmtna::cfg::Cfg;
/// use fmtna::cli::Cli;
/// use fmtna::engine;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let cli = Cli::parse();
/// let cfg: Cfg = confy::load("my_crate", "config")?;
/// let mut engine = engine::get_engine(cli, cfg)?;
/// engine.run()?;
/// # Ok(())
/// # }
/// ```
pub fn get_engine(cli: Cli, cfg: Cfg) -> anyhow::Result<Box<dyn Engine>> {
    match cli.command {
        Some(Command::Exclude(cli)) => exclude::get_engine(cli, cfg),
        Some(Command::Revert(cli)) => revert::get_engine(cli, cfg),
        None => default::get_engine(cli.args, cfg),
    }
}
