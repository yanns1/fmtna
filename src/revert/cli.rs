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
/// a file (in your config directory, in fmtna/history) each time
/// it runs.
/// You can go back to this file in case something went wrong,
/// modify it if desired and give it as argument to this
/// subcommand.
///
/// A revert operation can also go wrong, so a "second-order" backup
/// file will automatically be created in your config directory.
pub struct RevertCli {
    #[clap(verbatim_doc_comment)]
    /// The file specifying the filename changes to revert.
    pub history_file: PathBuf,
}
