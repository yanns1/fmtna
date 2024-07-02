use super::RevertCli;
use crate::cfg::Cfg;
use crate::engine::Engine;
use crate::prompt::{already_exist_prompt, error_prompt, AlreadyExistPromptOptions};
use crate::revert::data::Data;
use crate::utils::{backup, file_is_empty, get_history_dir_path, get_now_str, overwrite, skip};
use anyhow::anyhow;
use anyhow::Context;
use crossterm::style::Stylize;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

lazy_static! {
    static ref HISTORY_LINE_RE: Regex =
        Regex::new(r"\((?<op>.)\)\s+(?<from>.*)\s+->\s+(?<to>.*)\s*").unwrap();
}

pub fn get_engine(cli: RevertCli, cfg: Cfg) -> anyhow::Result<Box<dyn Engine>> {
    Ok(Box::new(RevertEngine::new(cli, cfg)?))
}

struct RevertEngine {
    data: Data,
    always_skip: bool,
    always_backup: bool,
    always_overwrite: bool,
}

impl RevertEngine {
    pub fn new(cli: RevertCli, cfg: Cfg) -> anyhow::Result<Self> {
        let data = Data::new(cli, cfg)?;
        Ok(Self {
            data,
            always_skip: false,
            always_backup: false,
            always_overwrite: false,
        })
    }
}

impl Engine for RevertEngine {
    fn run(&mut self) -> anyhow::Result<()> {
        // Create a backup file
        // ^^^^^^^^^^^^^^^^^^^^
        let mut history_path = get_history_dir_path()?;
        history_path.push(get_now_str());
        // Don't check if already exists as it shouldn't given the very precise time used for
        // the name.
        let history_file = File::create_new(history_path.clone())?;
        let mut history_writer = BufWriter::new(history_file);

        // Process lines
        // ^^^^^^^^^^^^^
        let mut invalid_linenos: Vec<usize> = vec![];
        let file = File::open(self.data.history_file.clone())?;
        let reader = BufReader::new(file);
        for (line_no, line) in reader.lines().enumerate() {
            let line_no = line_no + 1; // because start at zero, but want to start at one
            let line = line?;
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            let caps = HISTORY_LINE_RE.captures(&line);
            if caps.is_none() {
                invalid_linenos.push(line_no);
                continue;
            }
            let caps = caps.unwrap();
            let op = &caps["op"];
            let from = PathBuf::from(&caps["from"]);
            let to = PathBuf::from(&caps["to"]);
            let from_str = from.to_string_lossy();
            let to_str = to.to_string_lossy();

            if from == to {
                continue;
            }

            if !to.exists() {
                error_prompt(&to_str, "File doesn't exist.")?;
                continue;
            }

            if from.exists() {
                if self.always_skip {
                    skip(&to, &from, &mut history_writer)?;
                } else if self.always_backup {
                    backup(&to, &from, &mut history_writer)?;
                } else if self.always_overwrite {
                    overwrite(&to, &from, &mut history_writer)?;
                }

                match already_exist_prompt(&to_str, &from_str)? {
                    AlreadyExistPromptOptions::Skip => skip(&to, &from, &mut history_writer)?,
                    AlreadyExistPromptOptions::AlwaysSkip => {
                        skip(&to, &from, &mut history_writer)?;
                        self.always_skip = true;
                    }
                    AlreadyExistPromptOptions::Backup => backup(&to, &from, &mut history_writer)?,
                    AlreadyExistPromptOptions::AlwaysBackup => {
                        backup(&to, &from, &mut history_writer)?;
                        self.always_backup = true;
                    }
                    AlreadyExistPromptOptions::Overwrite => {
                        overwrite(&to, &from, &mut history_writer)?
                    }
                    AlreadyExistPromptOptions::AlwaysOverwrite => {
                        overwrite(&to, &from, &mut history_writer)?;
                        self.always_overwrite = true;
                    }
                };

                continue;
            }

            match op {
                "d" | "b" | "o" => {
                    let res = fs::rename(to.clone(), from.clone());
                    match res {
                        Ok(_) => {
                            let recap_line = format!("(d) {} -> {}", to_str, from_str);
                            println!("{}", recap_line.clone().dark_grey());
                            writeln!(history_writer, "{}", recap_line)
                                .with_context(|| "Failed to write to history file.")?;
                        }
                        Err(err) => {
                            error_prompt(&to_str, &format!("Failed to rename. {}", err)[..])?;
                        }
                    }
                }
                "s" => {
                    // Nothing to do
                }
                _ => {
                    invalid_linenos.push(line_no);
                }
            }
        }

        // Flush the BufWriter before checking if the history file is empty or not
        history_writer.flush()?;

        // Remove backup file if nothing was written to it.
        // Could theorically avoid making it in the first place,
        // but too unconvenient.
        if file_is_empty(&history_path)? {
            fs::remove_file(&history_path)?;
        }

        if !invalid_linenos.is_empty() {
            return Err(anyhow!(
                "Ignored {} invalid lines with line numbers {:?}, in {}.",
                invalid_linenos.len(),
                invalid_linenos,
                self.data.history_file.clone().to_string_lossy()
            ));
        }

        Ok(())
    }
}
