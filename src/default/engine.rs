use anyhow::Context;
use std::fs::File;
use std::io::{BufWriter, Write};

use super::cli::DefaultArgs;
use super::data::Data;
use crate::cfg::Cfg;
use crate::engine::Engine;
use crate::naming_conventions::apply_nc;
use crate::utils::{
    file_is_empty, get_backups_dir_path, get_history_dir_path, get_now_str, get_stdin_line_input,
    INDENT,
};
use core::panic;
use crossterm::style::Stylize;
use path_absolutize::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const HELP: &str = "[s]kip : Do nothing and continue.
[S]kip all : [s]kip for the current conflict and all further conflicts.
[b]ackup : Move the existing file in BACKUP_DIR, then rename the file supposed to be renamed.
[B]ackup all : [b]ackup for the current conflict and all further conflicts.
[o]verwrite : Rename anyway, overwriting the existing file in the process (beware data loss!).
[O]verwrite all : [o]verwrite for the current conflict and all further conflicts.";

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

    fn run_error_interaction<W: Write>(
        &self,
        f: &Path,
        err_mess: &str,
        history_writer: &mut W,
    ) -> anyhow::Result<()> {
        let path = f.to_string_lossy();
        let prompt = format!(
            "(?) {}: {}\n{}Enter a key to continue: ",
            path.red(),
            err_mess,
            INDENT
        );
        let valid_inputs: Vec<&str> = vec![];
        let _ = get_stdin_line_input(&prompt, &valid_inputs, None, None)?;
        let recap_line = format!("(e) {}: {}", path, err_mess);
        println!("{}", recap_line.clone().dark_red());
        writeln!(history_writer, "{}", recap_line)
            .with_context(|| "Failed to write to history file.")?;

        Ok(())
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

    fn skip<W: Write>(
        &mut self,
        path: &Path,
        new_path: &Path,
        history_writer: &mut W,
    ) -> anyhow::Result<()> {
        let recap_line = format!(
            "(s) {} -> {}",
            path.to_string_lossy(),
            new_path.to_string_lossy()
        );
        println!("{}", recap_line.clone().dark_blue());
        writeln!(history_writer, "{}", recap_line)
            .with_context(|| "Failed to write to history file.")?;

        Ok(())
    }

    fn backup<W: Write>(
        &mut self,
        path: &Path,
        new_path: &Path,
        history_writer: &mut W,
    ) -> anyhow::Result<()> {
        // Figure out the backup's filename
        // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        let mut new_name;
        let file_stem = new_path
            .file_stem()
            .with_context(|| "Expected new file to have a stem.")?;
        new_name = format!("{}_backup_{}", file_stem.to_string_lossy(), get_now_str());
        if let Some(extension) = new_path.extension() {
            new_name.push('.');
            new_name.push_str(&extension.to_string_lossy());
        }
        let mut backup_path = get_backups_dir_path()?;
        backup_path.push(new_name);

        // Make the backup
        // ^^^^^^^^^^^^^^^
        fs::rename(new_path, backup_path.clone()).with_context(|| {
            format!(
                "Failed to backup! Couldn't move {} to {}.",
                new_path.display(),
                backup_path.display()
            )
        })?;

        fs::rename(path, new_path).with_context(|| "Failed to rename.")?;

        // Report to the user
        // ^^^^^^^^^^^^^^^^^^
        let recap_line = format!(
            "(b) {} -> {}",
            path.to_string_lossy(),
            new_path.to_string_lossy()
        );
        println!("{}", recap_line.clone().dark_green());
        writeln!(history_writer, "{}", recap_line)
            .with_context(|| "Failed to write to history file.")?;

        Ok(())
    }

    fn overwrite<W: Write>(
        &mut self,
        path: &Path,
        new_path: &Path,
        history_writer: &mut W,
    ) -> anyhow::Result<()> {
        fs::rename(path, new_path).with_context(|| "Failed to rename.")?;

        let recap_line = format!(
            "(o) {} -> {}",
            path.to_string_lossy(),
            new_path.to_string_lossy()
        );
        println!("{}", recap_line.clone().dark_yellow());
        writeln!(history_writer, "{}", recap_line)
            .with_context(|| "Failed to write to history file.")?;

        Ok(())
    }

    fn process_file<W: Write>(&mut self, f: PathBuf, history_writer: &mut W) -> anyhow::Result<()> {
        if self.should_exclude(&f) {
            return Ok(());
        }

        match self.change_stem_of_file(&f) {
            ChangeStemResult::FileDoesntExist => {
                let f = f.absolutize()?;
                self.run_error_interaction(&f, "File doesn't exist.", history_writer)?;
            }
            ChangeStemResult::FailedToRetrieveFileStem => {
                let f = f.absolutize()?;
                self.run_error_interaction(&f, "Failed to find the stem.", history_writer)?;
            }
            ChangeStemResult::FileHasInvalidUnicode => {
                let f = f.absolutize()?;
                self.run_error_interaction(
                    &f,
                    "File contains invalid unicode characters.",
                    history_writer,
                )?;
            }
            ChangeStemResult::FailedToAbsolutizeFile(err) => {
                self.run_error_interaction(
                    &f,
                    &format!("Failed to absolutize. {}", err)[..],
                    history_writer,
                )?;
            }
            ChangeStemResult::FileHasNoParentDirectory => {
                let f = f.absolutize()?;
                self.run_error_interaction(&f, "File has no parent directory.", history_writer)?;
            }
            ChangeStemResult::NewFileAlreadyExist(new_file) => {
                let f = f.absolutize()?;

                if self.always_skip {
                    self.skip(&f, &new_file, history_writer)?;
                    return Ok(());
                } else if self.always_backup {
                    self.backup(&f, &new_file, history_writer)?;
                    return Ok(());
                } else if self.always_overwrite {
                    self.overwrite(&new_file, &new_file, history_writer)?;
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
                let input = get_stdin_line_input(&prompt, &valid_inputs, Some("h"), Some(HELP))?;
                match &input[..] {
                    "s" => self.skip(&f, &new_file, history_writer)?,
                    "S" => {
                        self.skip(&f, &new_file, history_writer)?;
                        self.always_skip = true;
                    }
                    "b" => self.backup(&f, &new_file, history_writer)?,
                    "B" => {
                        self.backup(&f, &new_file, history_writer)?;
                        self.always_backup = true;
                    }
                    "o" => self.overwrite(&f, &new_file, history_writer)?,
                    "O" => {
                        self.overwrite(&f, &new_file, history_writer)?;
                        self.always_overwrite = true;
                    }
                    _ => {
                        panic!("utils::get_stdin_line_input didn't do what it is supposed to!")
                    }
                };
            }
            ChangeStemResult::FailedToRename(err) => {
                self.run_error_interaction(
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
