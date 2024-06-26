use anyhow::Context;
use clap::crate_name;
use confy::get_configuration_file_path;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub const INDENT: &str = "    ";

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

#[cfg(test)]
pub mod test {
    use std::path::PathBuf;

    pub fn get_tmp_dir() -> PathBuf {
        let mut tmp_dir = std::env::current_dir().unwrap();
        tmp_dir.push(".tmp");
        tmp_dir
    }
}
