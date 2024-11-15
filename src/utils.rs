//! Utilities.

use crate::paths::BACKUP_DIR_PATH;
use anyhow::Context;
use crossterm::style::Stylize;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

/// Removes the newline (in a cross-platfrom way) at the end of `s` if there is one.
///
/// # Parameters
///
/// - `s`
pub fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

/// Returns whether the file at path `p` is empty.
///
/// # Parameters
///
/// - `p`
pub fn file_is_empty(p: &Path) -> io::Result<bool> {
    fs::metadata(p).map(|metadata| metadata.len() == 0)
}

/// Returns the current (local) date in format `%Y%m%d_%H%M%S%.9f`.
pub fn get_now_str() -> String {
    chrono::Local::now().format("%Y%m%d_%H%M%S%.9f").to_string()
}

/// Skips filename rewriting when conflict encountered, i.e. when `new_path`
/// points to an existing file.
///
/// Does nothing apart from writing feedback into stdout and `history_writer` in the form of:
///
/// ```text
/// (s) <link> -> <target>
/// ```
///
/// in dark blue (only for stdout).
///
/// # Parameters
///
/// - `path`: The path you are trying to rewrite.
/// - `new_path`: The path you want to rewrite into, but where an existing file
///     already exists.
/// - `history_writer`: Where to write feeback to, in addition to stdout.
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

/// Backs up the existing file at path `new_path`, then rewrites `path`
/// into `new_path`.
///
/// Finally, writes feeback into stdout and `history_writer` in the form of:
///
/// ```text
/// (b) <link> -> <target>
/// ```
///
/// in dark green (only for stdout).
///
/// # Parameters
///
/// - `path`: The path you are trying to rewrite.
/// - `new_path`: The path you want to rewrite into, but where an existing file
///     already exists.
/// - `history_writer`: Where to write feeback to, in addition to stdout.
///
/// # Errors
///
/// Fails when:
///
/// - The existing file fails to be backed up, i.e. fails to be moved
///   to the backup directory.
/// - The rewriting/renaming fails.
/// - Writing into `history_writer` fails.
///
/// These are `anyhow` errors, so most of the time, you just want to
/// propagate them.
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
    let mut backup_path = BACKUP_DIR_PATH.clone();
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

/// Overwrites existing file at path `new_path` by rewriting
/// `path` into it directly.
///
/// Finally, writes feeback into stdout and `history_writer` in the form of:
///
/// ```text
/// (o) <link> -> <target>
/// ```
///
/// in dark red (only for stdout).
///
/// # Parameters
///
/// - `path`: The path you are trying to rewrite.
/// - `new_path`: The path you want to rewrite into, but where an existing file
///     already exists.
/// - `history_writer`: Where to write feeback to, in addition to stdout.
///
/// # Errors
///
/// Fails when:
///
/// - The existing file fails to be removed.
/// - The rewriting/renaming fails.
/// - Writing into `history_writer` fails.
///
/// These are `anyhow` errors, so most of the time, you just want to
/// propagate them.
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
