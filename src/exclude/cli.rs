use clap::Args;
use clap::Subcommand;

use super::add::AddCli;
use super::del::DelCli;
use super::edit::EditCli;

#[derive(Args, Clone, Debug, PartialEq, Eq)]
#[clap(verbatim_doc_comment)]
/// Exclude filenames matching the given patterns when formatting.
///
/// Exclude patterns are specified in the configuration file exclude.txt.
/// This subcommand allows to add/remove entries to/from this file from the
/// command-line, or open it for edition using your favorite editor.
pub struct ExcludeCli {
    #[command(subcommand)]
    pub command: ExcludeCommand,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum ExcludeCommand {
    Add(AddCli),
    Del(DelCli),
    Edit(EditCli),
}
