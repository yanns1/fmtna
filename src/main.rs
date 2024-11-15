use std::fs;

use clap::{crate_name, Parser};
use fmtna::cfg::Cfg;
use fmtna::cli::Cli;
use fmtna::engine::get_engine;
use fmtna::paths::{BACKUP_DIR_PATH, EXCLUDE_FILE_PATH, HISTORY_DIR_PATH};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg: Cfg = confy::load(crate_name!(), crate_name!())?;
    let exclude_file_path = &*EXCLUDE_FILE_PATH;

    if !exclude_file_path.exists() {
        fs::copy("templates/exclude.txt", exclude_file_path)?;
    }

    let history_dir_path = &*HISTORY_DIR_PATH;
    if !history_dir_path.exists() {
        fs::create_dir(history_dir_path)?;
    }

    let backup_dir_path = &*BACKUP_DIR_PATH;
    if !backup_dir_path.exists() {
        fs::create_dir(backup_dir_path)?;
    }

    let mut engine = get_engine(cli, cfg)?;
    engine.run()
}
