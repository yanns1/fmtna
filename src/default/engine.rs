use anyhow::Context;
use std::fs::File;
use std::io::{BufWriter, Write};

use super::cli::DefaultArgs;
use super::data::Data;
use crate::cfg::Cfg;
use crate::engine::Engine;
use crate::naming_conventions::apply_nc;
use crate::utils::{
    backup, file_is_empty, get_history_dir_path, get_now_str, get_stdin_line_input, overwrite,
    run_error_interaction, skip, CONFLICT_HELP, INDENT,
};
use core::panic;
use crossterm::style::Stylize;
use path_absolutize::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn get_engine(cli: DefaultArgs, cfg: Cfg) -> anyhow::Result<Box<dyn Engine>> {
    Ok(Box::new(DefaultEngine::new(cli, cfg)?))
}

struct DefaultEngine {
    data: Data,
    always_skip: bool,
    always_backup: bool,
    always_overwrite: bool,
}

#[derive(Debug)]
enum ChangeStemResult {
    FileDoesntExist,
    FailedToRetrieveFileStem,
    FileHasInvalidUnicode,
    FailedToAbsolutizeFile(std::io::Error),
    FileHasNoParentDirectory,
    NewFileAlreadyExist(PathBuf),
    FailedToRename(std::io::Error),
    NoNeedToRename,
    Ok(PathBuf),
}

impl DefaultEngine {
    pub fn new(cli: DefaultArgs, cfg: Cfg) -> anyhow::Result<Self> {
        let data = Data::new(cli, cfg)?;
        Ok(Self {
            data,
            always_skip: false,
            always_backup: false,
            always_overwrite: false,
        })
    }

    fn change_stem_of_file(&self, file: &Path) -> ChangeStemResult {
        if !file.exists() {
            return ChangeStemResult::FileDoesntExist;
        }

        let file_stem = file.file_stem();
        if file_stem.is_none() {
            return ChangeStemResult::FailedToRetrieveFileStem;
        }
        let file_stem = file_stem.unwrap().to_str();
        if file_stem.is_none() {
            return ChangeStemResult::FileHasInvalidUnicode;
        }
        let file_stem = file_stem.unwrap();

        let file = file.absolutize();
        if let Err(err) = file {
            return ChangeStemResult::FailedToAbsolutizeFile(err);
        }
        let file = file.unwrap();
        let parent_dir = file.parent();
        if parent_dir.is_none() {
            return ChangeStemResult::FileHasNoParentDirectory;
        }
        let parent_dir = parent_dir.unwrap();

        let mut new_filename = apply_nc(
            &self.data.naming_convention,
            file_stem,
            self.data.keep_dots,
            self.data.keep_special_chars,
            self.data.keep_unicode,
        );

        if let Some(ext) = file.extension() {
            new_filename.push('.');
            new_filename.push_str(&ext.to_string_lossy());
        }
        let mut new_file = parent_dir.to_owned();
        new_file.push(new_filename);

        if new_file == file {
            return ChangeStemResult::NoNeedToRename;
        }

        if new_file.exists() {
            return ChangeStemResult::NewFileAlreadyExist(new_file);
        }

        let res = fs::rename(file, &new_file);
        if let Err(err) = res {
            return ChangeStemResult::FailedToRename(err);
        }

        ChangeStemResult::Ok(new_file)
    }

    fn should_exclude(&self, file: &Path) -> bool {
        if let Some(filename) = file.file_name() {
            let filename = filename.to_string_lossy();
            for re in &self.data.exclude_regexes {
                if re.is_match(&filename) {
                    return true;
                }
            }

            return false;
        }

        true
    }

    fn process_file<W: Write>(&mut self, f: PathBuf, history_writer: &mut W) -> anyhow::Result<()> {
        if self.should_exclude(&f) {
            return Ok(());
        }

        match self.change_stem_of_file(&f) {
            ChangeStemResult::FileDoesntExist => {
                let f = f.absolutize()?;
                run_error_interaction(&f, "File doesn't exist.", history_writer)?;
            }
            ChangeStemResult::FailedToRetrieveFileStem => {
                let f = f.absolutize()?;
                run_error_interaction(&f, "Failed to find the stem.", history_writer)?;
            }
            ChangeStemResult::FileHasInvalidUnicode => {
                let f = f.absolutize()?;
                run_error_interaction(
                    &f,
                    "File contains invalid unicode characters.",
                    history_writer,
                )?;
            }
            ChangeStemResult::FailedToAbsolutizeFile(err) => {
                run_error_interaction(
                    &f,
                    &format!("Failed to absolutize. {}", err)[..],
                    history_writer,
                )?;
            }
            ChangeStemResult::FileHasNoParentDirectory => {
                let f = f.absolutize()?;
                run_error_interaction(&f, "File has no parent directory.", history_writer)?;
            }
            ChangeStemResult::NewFileAlreadyExist(new_file) => {
                let f = f.absolutize()?;

                if self.always_skip {
                    skip(&f, &new_file, history_writer)?;
                    return Ok(());
                } else if self.always_backup {
                    backup(&f, &new_file, history_writer)?;
                    return Ok(());
                } else if self.always_overwrite {
                    overwrite(&new_file, &new_file, history_writer)?;
                    return Ok(());
                }

                let err_mess = "New file already exists.";
                let prompt = format!(
                    "(?) {} -> {}: {}\n{}[s]kip [S]kip all [b]ackup [B]ackup all [o]verwrite [O]verwrite all [h]elp: ",
                    f.to_string_lossy().red(),
                    new_file.to_string_lossy().red(),
                    err_mess,
                    INDENT
                );
                let valid_inputs: Vec<&str> = vec!["s", "S", "b", "B", "o", "O"];
                let input =
                    get_stdin_line_input(&prompt, &valid_inputs, Some("h"), Some(CONFLICT_HELP))?;
                match &input[..] {
                    "s" => skip(&f, &new_file, history_writer)?,
                    "S" => {
                        skip(&f, &new_file, history_writer)?;
                        self.always_skip = true;
                    }
                    "b" => backup(&f, &new_file, history_writer)?,
                    "B" => {
                        backup(&f, &new_file, history_writer)?;
                        self.always_backup = true;
                    }
                    "o" => overwrite(&f, &new_file, history_writer)?,
                    "O" => {
                        overwrite(&f, &new_file, history_writer)?;
                        self.always_overwrite = true;
                    }
                    _ => {
                        panic!("utils::get_stdin_line_input didn't do what it is supposed to!")
                    }
                };
            }
            ChangeStemResult::FailedToRename(err) => {
                run_error_interaction(
                    &f,
                    &format!("Failed to rename. {}", err)[..],
                    history_writer,
                )?;
            }
            ChangeStemResult::NoNeedToRename => {
                if self.data.recursive && !f.is_symlink() && f.is_dir() {
                    for entry in WalkDir::new(f)
                        .min_depth(1)
                        .into_iter()
                        .filter_map(|e| e.ok())
                    {
                        self.data.files.push(entry.path().to_owned());
                    }
                }
            }
            ChangeStemResult::Ok(new_file) => {
                let f = f.absolutize()?;
                let path = f.to_string_lossy();
                let new_path = new_file.to_string_lossy();

                let recap_line = format!("(d) {} -> {}", path, new_path);
                println!("{}", recap_line.clone().dark_grey());
                writeln!(history_writer, "{}", recap_line)
                    .with_context(|| "Failed to write to history file.")?;

                if self.data.recursive && !new_file.is_symlink() && new_file.is_dir() {
                    for entry in WalkDir::new(new_file)
                        .min_depth(1)
                        .into_iter()
                        .filter_map(|e| e.ok())
                    {
                        self.data.files.push(entry.path().to_owned());
                    }
                }
            }
        }

        Ok(())
    }
}

impl Engine for DefaultEngine {
    fn run(&mut self) -> anyhow::Result<()> {
        // Create a backup file
        // ^^^^^^^^^^^^^^^^^^^^
        let mut history_path = get_history_dir_path()?;
        history_path.push(get_now_str());
        // Don't check if already exists as it shouldn't given the very precise time used for
        // the name.
        let history_file = File::create_new(history_path.clone())?;
        let mut history_writer = BufWriter::new(history_file);

        // Process files
        // ^^^^^^^^^^^^^
        while let Some(f) = self.data.files.pop() {
            self.process_file(f, &mut history_writer)?;
        }

        // Flush the BufWriter before checking if the history file is empty or not
        history_writer.flush()?;

        // Remove backup file if nothing was written to it.
        // Could theorically avoid making it in the first place,
        // but too unconvenient.
        if file_is_empty(&history_path)? {
            fs::remove_file(&history_path)?;
        }

        Ok(())
    }
}
