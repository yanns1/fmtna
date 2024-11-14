use std::path::PathBuf;

use clap::Args;

use crate::naming_conventions::NamingConvention;

#[derive(Debug, Args)]
pub struct DefaultArgs {
    /// A list of files (of any kind) for which to format the name.
    ///
    /// If no file is given, nothing will happen and the program will exit gracefully.
    #[clap(verbatim_doc_comment)]
    pub files: Vec<PathBuf>,

    /// The naming convention to use.
    ///
    /// The default is "snake_case".
    /// If one is specified in the config file, it will be used instead.
    #[clap(verbatim_doc_comment)]
    #[arg(short, long)]
    pub naming_convention: Option<NamingConvention>,

    /// Recursively format filenames within directories.
    ///
    /// For arguments that are directories, the default is to treat them like
    /// any other file, that is format their names.
    /// By using this flag, every file (directories included) within each of
    /// the directories will be formatted as well.
    #[clap(verbatim_doc_comment)]
    #[arg(short, long)]
    pub recursive: bool,

    /// Don't treat dots as separators, let them as is.
    ///
    /// A separator is a character indicating a break between words.
    /// The characters "_", "-", "." and spaces are considered separators
    /// and may change according to the chosen naming convention, unless
    /// this flag is used.
    #[clap(verbatim_doc_comment)]
    #[arg(long)]
    pub keep_dots: bool,

    /// Keep special characters.
    ///
    /// By special characters we mean characters that are neither alphanumeric
    /// nor separators ("_", "-", "." and spaces).
    /// If not set, special characters are removed with the exception of some
    /// accented letters that are replaced by their non-accented variants.
    #[clap(verbatim_doc_comment)]
    #[arg(long)]
    pub keep_special_chars: bool,

    /// Keep Unicode (more precisely, non-ASCII) characters.
    ///
    /// When not set, convert unicode characters to their closest ASCII
    /// counterparts using <https://crates.io/crates/unidecode>.
    #[clap(verbatim_doc_comment)]
    #[arg(long)]
    pub keep_unicode: bool,
}
