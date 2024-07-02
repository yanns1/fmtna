use anyhow::Context;
use crossterm::style::Stylize;
use std::io;
use std::io::Write;

const INDENT: &str = "    ";

const CONFLICT_HELP: &str = "[s]kip : Do nothing and continue.
[S]kip all : [s]kip for the current conflict and all further conflicts.
[b]ackup : Move the existing file in the backup directory, then rename the file supposed to be renamed.
[B]ackup all : [b]ackup for the current conflict and all further conflicts.
[o]verwrite : Rename anyway, overwriting the existing file in the process (beware data loss!).
[O]verwrite all : [o]verwrite for the current conflict and all further conflicts.";

fn get_stdin_line_input() -> anyhow::Result<String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .with_context(|| "Error reading stdin input.")?;
    // Need this because the newline of Enter is included in the input
    input.truncate(input.len() - 1);

    Ok(input)
}

trait PromptOptions {
    fn match_input(input: &str) -> Option<Self>
    where
        Self: Sized;
    fn get_valid_inputs() -> Vec<String>;
}

fn prompt_option<PO: PromptOptions>(
    mess: &str,
    help_input: Option<&str>,
    help_mess: Option<&str>,
) -> anyhow::Result<PO> {
    let has_help = help_input.is_some() && help_mess.is_some();
    let help_input = help_input.unwrap_or("");
    let help_mess = help_mess.unwrap_or("");

    loop {
        print!("{}", mess);
        io::stdout().flush()?;
        let input = get_stdin_line_input()?;

        if let Some(opt) = PO::match_input(&input) {
            return Ok(opt);
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
                PO::get_valid_inputs().join(", "),
                help_key,
            );
        }
    }
}

struct ErrorPromptOptions {}

impl PromptOptions for ErrorPromptOptions {
    fn match_input(input: &str) -> Option<Self> {
        let _ = input;
        Some(ErrorPromptOptions {})
    }

    fn get_valid_inputs() -> Vec<String> {
        vec![]
    }
}

pub fn error_prompt(path_str: &str, err_mess: &str) -> anyhow::Result<()> {
    let prompt_mess = format!(
        "(?) {}: {}\n{}Enter a key to continue: ",
        path_str.red(),
        err_mess,
        INDENT
    );
    let _ = prompt_option::<ErrorPromptOptions>(&prompt_mess, None, None)?;

    Ok(())
}

pub enum AlreadyExistPromptOptions {
    Skip,
    AlwaysSkip,
    Backup,
    AlwaysBackup,
    Overwrite,
    AlwaysOverwrite,
}

impl PromptOptions for AlreadyExistPromptOptions {
    fn match_input(input: &str) -> Option<Self> {
        match input {
            "s" => Some(AlreadyExistPromptOptions::Skip),
            "S" => Some(AlreadyExistPromptOptions::AlwaysSkip),
            "b" => Some(AlreadyExistPromptOptions::Backup),
            "B" => Some(AlreadyExistPromptOptions::AlwaysBackup),
            "o" => Some(AlreadyExistPromptOptions::Overwrite),
            "O" => Some(AlreadyExistPromptOptions::AlwaysOverwrite),
            _ => None,
        }
    }

    fn get_valid_inputs() -> Vec<String> {
        vec![
            String::from("s"),
            String::from("S"),
            String::from("b"),
            String::from("B"),
            String::from("o"),
            String::from("O"),
        ]
    }
}

pub fn already_exist_prompt(
    path_str: &str,
    new_path_str: &str,
) -> anyhow::Result<AlreadyExistPromptOptions> {
    let prompt_mess = format!(
        "(?) {} -> {}: New file already exists.
{}[s]kip [S]kip all [b]ackup [B]ackup all [o]verwrite [O]verwrite all [h]elp: ",
        path_str.red(),
        new_path_str.red(),
        INDENT
    );
    let input =
        prompt_option::<AlreadyExistPromptOptions>(&prompt_mess, Some("h"), Some(CONFLICT_HELP))?;

    Ok(input)
}
