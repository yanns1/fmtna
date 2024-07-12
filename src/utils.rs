use anyhow::Context;
use clap::crate_name;
use confy::get_configuration_file_path;
use crossterm::style::Stylize;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

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
    chrono::Local::now().format("%Y%m%d_%H%M%S%.9f").to_string()
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
