use crate::cfg::Cfg;
use crate::cli::Cli;
use crate::cli::Command;
use crate::default;
use crate::exclude;
use crate::revert;

pub trait Engine {
    fn run(&mut self) -> anyhow::Result<()>;
}

pub fn get_engine(cli: Cli, cfg: Cfg) -> anyhow::Result<Box<dyn Engine>> {
    match cli.command {
        Some(Command::Exclude(cli)) => exclude::get_engine(cli, cfg),
        Some(Command::Revert(cli)) => revert::get_engine(cli, cfg),
        None => default::get_engine(cli.args, cfg),
    }
}
