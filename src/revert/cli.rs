use std::path::PathBuf;

use clap::Args;

#[derive(Args, Clone, Debug, PartialEq, Eq)]
#[clap(verbatim_doc_comment)]
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
/// You can even make a file on your own provided it has the
/// right format, but you shouldn't have to do this.
///
/// A revert operation can also go wrong, so a "second-order" backup
/// file will automatically be created in your config directory.
pub struct RevertCli {
    #[clap(verbatim_doc_comment)]
    /// The file specifying the filename changes to revert.
    pub backup_file: PathBuf,
}
