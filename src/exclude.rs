mod add;
mod cli;
mod del;
mod edit;
mod engine;
use clap::crate_name;
pub use cli::ExcludeCli;
use confy::get_configuration_file_path;
pub use engine::get_engine;
use std::path::PathBuf;

pub fn get_exclude_file_path() -> anyhow::Result<PathBuf> {
    let mut exclude_file_path = get_configuration_file_path(crate_name!(), crate_name!())?;
    exclude_file_path.set_file_name("exclude.txt");
    Ok(exclude_file_path)
}
