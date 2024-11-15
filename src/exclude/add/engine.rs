use std::fs::File;
use std::fs::OpenOptions;

use std::io::Write;
use std::io::{BufRead, BufReader};

use anyhow::Context;

use super::cli::AddCli;
use super::data::Data;
use crate::cfg::Cfg;
use crate::engine::Engine;
use crate::utils::get_exclude_file_path;

/// Returns the engine for the add subcommand, parameterized by `cli` and `cfg`.
///
/// # Parameters
///
/// - `cli`: The CLI arguments.
/// - `cfg`: The configuration values.
///
/// # Returns
///
/// The parametrized engine for running the add subcommand's logic, or an
/// error if engine creation failed.
pub fn get_engine(cli: AddCli, cfg: Cfg) -> anyhow::Result<Box<dyn Engine>> {
    Ok(Box::new(AddEngine::new(cli, cfg)?))
}

struct AddEngine {
    data: Data,
}

impl AddEngine {
    pub fn new(cli: AddCli, cfg: Cfg) -> anyhow::Result<Self> {
        let data = Data::new(cli, cfg)?;
        Ok(Self { data })
    }
}

impl Engine for AddEngine {
    fn run(&mut self) -> anyhow::Result<()> {
        let exclude_file_path = get_exclude_file_path()?;

        // Check if pattern to add already is in exclude file
        if exclude_file_path.exists() {
            let file = File::open(exclude_file_path.clone())?;
            let reader = BufReader::new(file);
            for (line_no, line) in reader.lines().enumerate() {
                let line = line?;

                if line.is_empty() || line.starts_with("//") {
                    continue;
                }

                if line == self.data.exclude_pattern {
                    println!(
                        "Exclude pattern already in {}, line {}. Nothing done.",
                        exclude_file_path.to_string_lossy(),
                        line_no
                    );
                    return Ok(());
                }
            }
        }

        // Append new pattern to exclude file
        let mut exclude_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(exclude_file_path.clone())
            .with_context(|| {
                format!(
                    "Failed to open exclude file ({}).",
                    exclude_file_path.to_string_lossy()
                )
            })?;

        writeln!(exclude_file, "{}", self.data.exclude_pattern).with_context(|| {
            format!(
                "Failed to write to exclude file ({}).",
                exclude_file_path.to_string_lossy()
            )
        })?;

        Ok(())
    }
}
