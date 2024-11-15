use super::add;
use super::cli::ExcludeCommand;
use super::del;
use super::edit;
use super::ExcludeCli;
use crate::cfg::Cfg;
use crate::engine::Engine;

/// Returns the engine for the exclude subcommand, parameterized by `cli` and `cfg`.
///
/// # Parameters
///
/// - `cli`: The CLI arguments.
/// - `cfg`: The configuration values.
///
/// # Returns
///
/// The parametrized engine for running the exclude subcommand's logic, or an
/// error if engine creation failed.
pub fn get_engine(cli: ExcludeCli, cfg: Cfg) -> anyhow::Result<Box<dyn Engine>> {
    match cli.command {
        ExcludeCommand::Add(cli) => add::get_engine(cli, cfg),
        ExcludeCommand::Del(cli) => del::get_engine(cli, cfg),
        ExcludeCommand::Edit(cli) => edit::get_engine(cli, cfg),
    }
}
