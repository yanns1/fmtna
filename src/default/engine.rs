use super::cli::DefaultArgs;
use super::data::Data;
use crate::cfg::Cfg;
use crate::engine::Engine;
use crate::naming_conventions::apply_nc;
use crate::utils::{self, INDENT};
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
        Ok(Self { data })
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

        // TODO: Handle dotfiles so has to send only the part after the dot the naming convetion
        let new_file_stem = apply_nc(
            &self.data.naming_convention,
            file_stem,
            self.data.keep_dots,
            self.data.keep_special_chars,
            self.data.keep_unicode,
        );

        let mut new_file = parent_dir.to_owned();
        new_file.push(new_file_stem);

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

    fn run_error_interaction(&self, f: &Path, err_mess: &str) -> anyhow::Result<()> {
        let path = f.to_string_lossy();
        let prompt = format!(
            "(?) {}: {}\n{}Enter a key to continue: ",
            path.red(),
            err_mess,
            INDENT
        );
        let valid_inputs: Vec<&str> = vec![];
        let _ = utils::get_stdin_line_input(&prompt, &valid_inputs, None, None, true)?;
        println!("{}", format!("(e) {}: {}", path, err_mess).dark_red());

        Ok(())
    }
}

impl Engine for DefaultEngine {
    fn run(&mut self) -> anyhow::Result<()> {
        while let Some(f) = self.data.files.pop() {
            match self.change_stem_of_file(&f) {
                ChangeStemResult::FileDoesntExist => {
                    let f = f.absolutize()?;
                    self.run_error_interaction(&f, "File doesn't exist.")?;
                }
                ChangeStemResult::FailedToRetrieveFileStem => {
                    let f = f.absolutize()?;
                    self.run_error_interaction(&f, "Failed to find the stem.")?;
                }
                ChangeStemResult::FileHasInvalidUnicode => {
                    let f = f.absolutize()?;
                    self.run_error_interaction(&f, "File contains invalid unicode characters.")?;
                }
                ChangeStemResult::FailedToAbsolutizeFile(err) => {
                    self.run_error_interaction(&f, &format!("Failed to absolutize. {}", err)[..])?;
                }
                ChangeStemResult::FileHasNoParentDirectory => {
                    let f = f.absolutize()?;
                    self.run_error_interaction(&f, "File has no parent directory.")?;
                }
                ChangeStemResult::NewFileAlreadyExist(new_file) => {
                    let f = f.absolutize()?;
                    let path = f.to_string_lossy();
                    let new_path = new_file.to_string_lossy();
                    let err_mess = "New file already exists.";
                    let prompt = format!(
                        "(?) {} -> {}: {}\n{}[s]kip [S]kip all [b]ackup [B]ackup all [o]verwrite [O]verwrite all [h]elp: ",
                        path.red(),
                        new_path.red(),
                        err_mess,
                        INDENT
                    );
                    let valid_inputs: Vec<&str> = vec!["s", "S", "b", "B", "o", "O"];
                    let input = utils::get_stdin_line_input(
                        &prompt,
                        &valid_inputs,
                        Some("h"),
                        Some(HELP),
                        true,
                    )?;
                    match &input[..] {
                        "s" => {
                            println!("{}", format!("(s) {} -> {}", path, new_path).dark_blue());
                            todo!();
                        }
                        "S" => {
                            println!("{}", format!("(s) {} -> {}", path, new_path).dark_blue());
                            todo!();
                        }
                        "b" => {
                            println!("{}", format!("(b) {} -> {}", path, new_path).dark_green());
                            todo!();
                        }
                        "B" => {
                            println!("{}", format!("(b) {} -> {}", path, new_path).dark_green());
                            todo!();
                        }
                        "o" => {
                            println!("{}", format!("(o) {} -> {}", path, new_path).dark_yellow());
                            todo!();
                        }
                        "O" => {
                            println!("{}", format!("(o) {} -> {}", path, new_path).dark_yellow());
                            todo!();
                        }
                        _ => {
                            panic!("utils::get_stdin_line_input didn't do what it is supposed to!")
                        }
                    };
                }
                ChangeStemResult::FailedToRename(err) => {
                    self.run_error_interaction(&f, &format!("Failed to rename. {}", err)[..])?;
                }
                ChangeStemResult::NoNeedToRename => {
                    if self.data.recursive && !f.is_symlink() && f.is_dir() {
                        for entry in WalkDir::new(f).into_iter().filter_map(|e| e.ok()) {
                            self.data.files.push(entry.path().to_owned());
                        }
                    }
                }
                ChangeStemResult::Ok(new_file) => {
                    let f = f.absolutize()?;
                    let path = f.to_string_lossy();
                    let new_path = new_file.to_string_lossy();
                    println!("{}", format!("(d) {} -> {}", path, new_path).dark_grey());

                    if self.data.recursive && !new_file.is_symlink() && new_file.is_dir() {
                        for entry in WalkDir::new(new_file).into_iter().filter_map(|e| e.ok()) {
                            self.data.files.push(entry.path().to_owned());
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
