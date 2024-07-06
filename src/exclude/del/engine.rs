use edit_distance::edit_distance;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom, Write};

use anyhow::Context;

use super::cli::DelCli;
use super::data::Data;
use crate::cfg::Cfg;
use crate::engine::Engine;
use crate::utils::get_exclude_file_path;
use tempfile::tempfile;

pub fn get_engine(cli: DelCli, cfg: Cfg) -> anyhow::Result<Box<dyn Engine>> {
    Ok(Box::new(DelEngine::new(cli, cfg)?))
}

struct DelEngine {
    data: Data,
}

impl DelEngine {
    pub fn new(cli: DelCli, cfg: Cfg) -> anyhow::Result<Self> {
        let data = Data::new(cli, cfg)?;
        Ok(Self { data })
    }
}

impl Engine for DelEngine {
    fn run(&mut self) -> anyhow::Result<()> {
        let exclude_file_path = get_exclude_file_path()?;

        if !exclude_file_path.exists() {
            println!(
                "Exclude file at path {} does not exist. Nothing done.",
                exclude_file_path.to_string_lossy()
            );
            return Ok(());
        }

        // Copy exclude file to tempfile, unless the line that contains the pattern to delete
        let exclude_file = OpenOptions::new()
            .read(true)
            .open(exclude_file_path.clone())
            .with_context(|| {
                format!(
                    "Failed to read exclude file ({}).",
                    exclude_file_path.to_string_lossy()
                )
            })?;
        let reader = BufReader::new(exclude_file);
        let mut tmp_file = tempfile().with_context(|| "Failed to create tempfile.")?;
        let mut min_dist: usize = usize::MAX;
        let mut closest_pattern = String::from("");
        let mut found = false;
        for line in reader.lines() {
            let line = line?;

            if line.is_empty() || line.starts_with("//") {
                writeln!(tmp_file, "{}", line).with_context(|| "Failed to write to tempfile.")?;
                continue;
            }

            if line == self.data.exclude_pattern {
                found = true;
                continue;
            }

            if !found {
                let dist = edit_distance(&line, &self.data.exclude_pattern);
                if dist < min_dist {
                    min_dist = dist;
                    closest_pattern.clone_from(&line);
                }
            }

            writeln!(tmp_file, "{}", line).with_context(|| "Failed to write to tempfile.")?;
        }

        // Report to use if pattern not found
        if !found {
            println!(
                "Didn't found pattern {} in exclude file.",
                self.data.exclude_pattern
            );
            if !closest_pattern.is_empty() {
                println!("Closest pattern found is {}", closest_pattern);
            }
            return Ok(());
        }

        // Copy tempfile back to exclude file
        tmp_file.seek(SeekFrom::Start(0))?;
        let mut exclude_file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(exclude_file_path.clone())
            .with_context(|| {
                format!(
                    "Failed to write to exclude file ({}).",
                    exclude_file_path.to_string_lossy()
                )
            })?;
        io::copy(&mut tmp_file, &mut exclude_file)
            .with_context(|| "Failed to copy tempfile back to exclude file.")?;

        Ok(())
    }
}
