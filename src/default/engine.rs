use std::io::{BufWriter, Write};

use super::cli::DefaultArgs;
use super::data::Data;
use crate::cfg::Cfg;
use crate::engine::Engine;
use crate::naming_conventions::apply_nc;
use crate::prompt::{already_exist_prompt, error_prompt, AlreadyExistPromptOptions};
use crate::utils::{backup, file_is_empty, get_history_dir_path, get_now_str, overwrite, skip};
use anyhow::Context;
use crossterm::style::Stylize;
use path_absolutize::*;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Returns the engine for the default subcommand, parameterized by `cli` and `cfg`.
///
/// # Parameters
///
/// - `cli`: The CLI arguments.
/// - `cfg`: The configuration values.
///
/// # Returns
///
/// The parametrized engine for running the default subcommand's logic, or an
/// error if engine creation failed.
pub fn get_engine(cli: DefaultArgs, cfg: Cfg) -> anyhow::Result<Box<dyn Engine>> {
    Ok(Box::new(DefaultEngine::new(cli, cfg)?))
}

struct DefaultEngine {
    data: Data,
    action: Option<Action>,
}

enum Action {
    Skip,
    Backup,
    Overwrite,
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
        Ok(Self { data, action: None })
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

        // because paths are case-insensitive on Windows
        if cfg!(windows) && new_filename.to_lowercase() == file_stem.to_lowercase() {
            return ChangeStemResult::NoNeedToRename;
        }

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
                let f_str = f.to_string_lossy();
                let err_mess = "File doesn't exist.";

                error_prompt(&f_str, err_mess)?;

                let recap_line = format!("(e) {}: {}", f_str, err_mess);
                println!("{}", recap_line.clone().dark_red());
                writeln!(history_writer, "{}", recap_line)
                    .with_context(|| "Failed to write to history file.")?;
            }
            ChangeStemResult::FailedToRetrieveFileStem => {
                let f = f.absolutize()?;
                let f_str = f.to_string_lossy();
                let err_mess = "Failed to find the stem.";

                error_prompt(&f_str, err_mess)?;

                let recap_line = format!("(e) {}: {}", f_str, err_mess);
                println!("{}", recap_line.clone().dark_red());
                writeln!(history_writer, "{}", recap_line)
                    .with_context(|| "Failed to write to history file.")?;
            }
            ChangeStemResult::FileHasInvalidUnicode => {
                let f = f.absolutize()?;
                let f_str = f.to_string_lossy();
                let err_mess = "File contains invalid unicode characters.";

                error_prompt(&f_str, err_mess)?;

                let recap_line = format!("(e) {}: {}", f_str, err_mess);
                println!("{}", recap_line.clone().dark_red());
                writeln!(history_writer, "{}", recap_line)
                    .with_context(|| "Failed to write to history file.")?;
            }
            ChangeStemResult::FailedToAbsolutizeFile(err) => {
                let f_str = f.to_string_lossy();
                let err_mess = format!("Failed to absolutize. {}", err);

                error_prompt(&f_str, &err_mess)?;

                let recap_line = format!("(e) {}: {}", f_str, err_mess);
                println!("{}", recap_line.clone().dark_red());
                writeln!(history_writer, "{}", recap_line)
                    .with_context(|| "Failed to write to history file.")?;
            }
            ChangeStemResult::FileHasNoParentDirectory => {
                let f = f.absolutize()?;
                let f_str = f.to_string_lossy();
                let err_mess = "File has no parent directory";

                error_prompt(&f_str, err_mess)?;

                let recap_line = format!("(e) {}: {}", f_str, err_mess);
                println!("{}", recap_line.clone().dark_red());
                writeln!(history_writer, "{}", recap_line)
                    .with_context(|| "Failed to write to history file.")?;
            }
            ChangeStemResult::NewFileAlreadyExist(new_f) => {
                let f = f.absolutize()?;

                if let Some(ref action) = self.action {
                    match action {
                        Action::Skip => skip(&f, &new_f, history_writer)?,
                        Action::Backup => backup(&f, &new_f, history_writer)?,
                        Action::Overwrite => overwrite(&new_f, &new_f, history_writer)?,
                    }
                    return Ok(());
                }

                let f_str = f.to_string_lossy();
                let new_f_str = new_f.to_string_lossy();
                match already_exist_prompt(&f_str, &new_f_str)? {
                    AlreadyExistPromptOptions::Skip => {
                        skip(&f, &new_f, history_writer)?;
                    }
                    AlreadyExistPromptOptions::AlwaysSkip => {
                        skip(&f, &new_f, history_writer)?;
                        self.action = Some(Action::Skip);
                    }
                    AlreadyExistPromptOptions::Backup => {
                        backup(&f, &new_f, history_writer)?;
                    }
                    AlreadyExistPromptOptions::AlwaysBackup => {
                        backup(&f, &new_f, history_writer)?;
                        self.action = Some(Action::Backup);
                    }
                    AlreadyExistPromptOptions::Overwrite => {
                        overwrite(&f, &new_f, history_writer)?;
                    }
                    AlreadyExistPromptOptions::AlwaysOverwrite => {
                        overwrite(&f, &new_f, history_writer)?;
                        self.action = Some(Action::Overwrite);
                    }
                };
            }
            ChangeStemResult::FailedToRename(err) => {
                let f_str = f.to_string_lossy();
                let err_mess = format!("Failed to rename. {}", err);

                error_prompt(&f_str, &err_mess)?;

                let recap_line = format!("(e) {}: {}", f_str, err_mess);
                println!("{}", recap_line.clone().dark_red());
                writeln!(history_writer, "{}", recap_line)
                    .with_context(|| "Failed to write to history file.")?;
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
            ChangeStemResult::Ok(new_f) => {
                let f = f.absolutize()?;
                let f_str = f.to_string_lossy();
                let new_f_str = new_f.to_string_lossy();

                let recap_line = format!("(d) {} -> {}", f_str, new_f_str);
                println!("{}", recap_line.clone().dark_grey());
                writeln!(history_writer, "{}", recap_line)
                    .with_context(|| "Failed to write to history file.")?;

                if self.data.recursive && !new_f.is_symlink() && new_f.is_dir() {
                    for entry in WalkDir::new(new_f)
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
