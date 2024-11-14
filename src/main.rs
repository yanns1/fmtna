use std::fs;

use clap::{crate_name, Parser};
use fmtna::cfg::Cfg;
use fmtna::cli::Cli;
use fmtna::engine::get_engine;
use fmtna::utils::{get_backups_dir_path, get_exclude_file_path, get_history_dir_path};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg: Cfg = confy::load(crate_name!(), crate_name!())?;
    let exclude_file_path = get_exclude_file_path()?;

    if !exclude_file_path.exists() {
        fs::copy("templates/exclude.txt", exclude_file_path)?;
    }

    let history_dir_path = get_history_dir_path()?;
    if !history_dir_path.exists() {
        fs::create_dir(history_dir_path)?;
    }

    let backups_dir_path = get_backups_dir_path()?;
    if !backups_dir_path.exists() {
        fs::create_dir(backups_dir_path)?;
    }

    let mut engine = get_engine(cli, cfg)?;
    engine.run()
}
