use anyhow::Context;
use clap::crate_name;
use confy::get_configuration_file_path;
use crossterm::style::Stylize;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub const INDENT: &str = "    ";

pub const CONFLICT_HELP: &str = "[s]kip : Do nothing and continue.
[S]kip all : [s]kip for the current conflict and all further conflicts.
[b]ackup : Move the existing file in the backup directory, then rename the file supposed to be renamed.
[B]ackup all : [b]ackup for the current conflict and all further conflicts.
[o]verwrite : Rename anyway, overwriting the existing file in the process (beware data loss!).
[O]verwrite all : [o]verwrite for the current conflict and all further conflicts.";

pub fn get_stdin_raw_line_input() -> anyhow::Result<String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .with_context(|| "Error reading stdin input.")?;
    // Need this because the newline of Enter is included in the input
    input.truncate(input.len() - 1);

    Ok(input)
}

pub fn get_stdin_line_input(
    prompt: &str,
    valid_inputs: &[&str],
    help_input: Option<&str>,
    help_mess: Option<&str>,
) -> anyhow::Result<String> {
    let has_help = help_input.is_some() && help_mess.is_some();
    let help_input = help_input.unwrap_or("");
    let help_mess = help_mess.unwrap_or("");

    loop {
        print!("{}", prompt);
        io::stdout().flush()?;
        let input = get_stdin_raw_line_input()?;

        if valid_inputs.is_empty() {
            return Ok(input);
        } else if let Some(pos) = valid_inputs.iter().position(|&i| i == input) {
            return Ok(valid_inputs[pos].to_owned());
        } else if has_help && input == help_input {
            println!("{INDENT}----------");
            for line in help_mess.lines() {
                println!("{INDENT}{}", line);
            }
            println!("{INDENT}----------");
        } else {
            let mut help_key = String::from("");
            if has_help {
                help_key = format!(", {}", help_input);
            }
            println!(
                "{INDENT}Wrong input! Valid inputs are: {}{}. Try again.",
                valid_inputs.join(", "),
                help_key,
            );
        }
    }
}

pub fn get_exclude_file_path() -> anyhow::Result<PathBuf> {
    let mut exclude_file_path = get_configuration_file_path(crate_name!(), crate_name!())?;
    exclude_file_path.set_file_name("exclude.txt");
    Ok(exclude_file_path)
}

pub fn get_history_dir_path() -> anyhow::Result<PathBuf> {
    let mut history_dir_path = get_configuration_file_path(crate_name!(), crate_name!())?
        .parent()
        .with_context(|| "Failed to get parent directory of configuration file.")?
        .to_owned();
    history_dir_path.push("history");
    Ok(history_dir_path)
}

pub fn get_backups_dir_path() -> anyhow::Result<PathBuf> {
    let mut backups_dir_path = get_configuration_file_path(crate_name!(), crate_name!())?
        .parent()
        .with_context(|| "Failed to get parent directory of configuration file.")?
        .to_owned();
    backups_dir_path.push("backups");
    Ok(backups_dir_path)
}

pub fn file_is_empty(p: &Path) -> io::Result<bool> {
    fs::metadata(p).map(|metadata| metadata.len() == 0)
}

pub fn get_now_str() -> String {
    chrono::Local::now().to_rfc3339()
}

pub fn run_error_interaction<W: Write>(
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

pub fn skip<W: Write>(path: &Path, new_path: &Path, history_writer: &mut W) -> anyhow::Result<()> {
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

pub fn backup<W: Write>(
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

pub fn overwrite<W: Write>(
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

#[cfg(test)]
pub mod test {
    use std::path::PathBuf;

    pub fn get_tmp_dir() -> PathBuf {
        let mut tmp_dir = std::env::current_dir().unwrap();
        tmp_dir.push(".tmp");
        tmp_dir
    }
}
