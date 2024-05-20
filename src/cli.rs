use clap::{Parser, Subcommand, ValueEnum};
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version)]
#[command(propagate_version = true)]
#[clap(verbatim_doc_comment)]
/// Format filenames according to a chosen naming convention.
///
/// For each file/path (of any kind) given as argument, change the filename
/// (i.e. the base of the path) according to the naming convention selected.
///
/// WARNING! This program is dangerous.
/// Changing filenames is error prone and may cause undesired overwrites
/// or consequences (some files are expected to have the name they have
/// and not something else!).
/// fmtna's solves these problems by:
///     1. Stopping you from executing the program as root.
///     2. Asking you what to do when conflicts happen (the program
///        wants to change a path to an already existing path).
///     3. Backing up the filename changes and allowing you to revert
///        the changes partially or completely.
///     4. Giving you ways to exclude some filenames from formatting.
/// Still, fmtna can't stop you from shooting yourself in the foot.
/// It can go as far as corrupting your system.
pub struct Cli {
    #[command(subcommand)]
    subcommands: Option<Subcommands>,

    /// A list of files (any kind of file) for which to format the name.
    ///
    /// If no file is given, nothing will happen and the program will exit gracefully.
    #[clap(verbatim_doc_comment)]
    files: Vec<PathBuf>,

    /// The naming convention to use.
    ///
    /// The default is "snake_case".
    /// If one is specified in the config file, it will be used instead.
    #[clap(verbatim_doc_comment)]
    #[arg(short, long)]
    naming_convention: Option<NamingConvention>,

    /// Recursively format filenames within directories.
    ///
    /// For arguments that are directories, the default is to treat them like
    /// any other file, that is format their names.
    /// By using this flog, every file (directories included) within each of
    /// the directories will be formatted as well.
    #[clap(verbatim_doc_comment)]
    #[arg(short, long)]
    recursive: bool,

    /// Don't treat dots as separators, let them as is.
    ///
    /// A separator is a character indicating a break between words.
    /// The characters "_", "-", "." and spaces are considered separators
    /// and may changed according to the chosen naming convention, unless
    /// this flog is used.
    #[clap(verbatim_doc_comment)]
    #[arg(long)]
    keep_dots: bool,

    /// Keep special characters.
    ///
    /// By special characters we mean characters that are neither alphanumeric
    /// nor separators ("_", "-", "." and spaces).
    /// If not set, special characters are removed with the exception of some
    /// accented letters that are replaced by their non-accented variants.
    #[clap(verbatim_doc_comment)]
    #[arg(long)]
    keep_special_chars: bool,
}

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize)]
pub enum NamingConvention {
    #[serde(rename = "camelCase")]
    #[value(name = "camelCase")]
    CamelCase,
    #[serde(rename = "kebab-case")]
    #[value(name = "kebab-case")]
    KebabCase,
    #[serde(rename = "snake_case")]
    #[value(name = "snake_case")]
    SnakeCase,
    #[serde(rename = "PascalCase")]
    #[value(name = "PascalCase")]
    PascalCase,
    #[serde(rename = "lower")]
    #[value(name = "lower")]
    Lower,
    #[serde(rename = "UPPER")]
    #[value(name = "UPPER")]
    Upper,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Subcommands {
    /// Exclude filenames matching the given patterns when formatting.
    ///
    /// Exclude patterns are specified in the configuration file exclude.txt.
    /// This subcommand allows to add/remove entries to/from this file from the
    /// command-line, or open it for edition it using your favorite editor.
    #[clap(verbatim_doc_comment)]
    Exclude {
        #[command(subcommand)]
        subcommands: ExcludeSubcommands,
    },

    /// Revert filename changes.
    ///
    /// This subcommand allows to revert previous runs of the program.
    /// This is useful when the changes are not the expected ones
    /// or have unexpected consequences.
    ///
    /// fmtna automatically backs up the filename changes in
    /// a file (in your config directory) each time it runs.
    /// The contents of the backup file is exactly the output printed
    /// to the terminal.
    /// You can go back to this file in case something went wrong,
    /// modify it if desired and give it as argument to this
    /// subcommand.
    /// You can even make a file on you own provided it has the
    /// right format, but you shouldn't have to do this.
    #[clap(verbatim_doc_comment)]
    Revert {
        #[clap(verbatim_doc_comment)]
        /// The file specifying the filename changes to revert.
        backup_file: PathBuf,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ExcludeSubcommands {
    /// Add a pattern to exclude.txt.
    #[clap(verbatim_doc_comment)]
    Add {
        /// A pattern to add to exclude.txt.
        ///
        /// If the pattern already is in exclude.txt,
        /// nothing will happen and you will be warned about it.
        #[clap(verbatim_doc_comment)]
        pattern: String,
    },

    /// Delete a pattern from exclude.txt.
    #[clap(verbatim_doc_comment)]
    Del {
        /// The pattern to delete from exclude.txt`.
        ///
        /// If the pattern is not found in exclude.txt,
        /// nothing will happen and you will be warned about it.
        /// Furthermore, the closest pattern found in the file
        /// will be proposed for deletion as a guess for
        /// what you really wanted to delete.
        #[clap(verbatim_doc_comment)]
        pattern: String,
    },

    /// Open exclude.txt for edition.
    ///
    /// By default the value of the environment variable $EDITOR is
    /// used as the editor with which to open exclude.txt.
    /// If editor is set in the config file, then it takes precedence.
    /// If the EDITOR argument to this subcommand is used, then it
    /// takes precedence over both the config and then environment
    /// variable.
    /// If none of the three options are set, "vi" will be used.
    #[clap(verbatim_doc_comment)]
    Edit {
        /// The editor with which to open exclude.txt.
        #[clap(verbatim_doc_comment)]
        editor: Option<String>,
    },
}
