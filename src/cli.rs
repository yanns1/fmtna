//! Everything related to the app's CLI.

use crate::default::DefaultArgs;
use crate::exclude::ExcludeCli;
use crate::revert::RevertCli;
use clap::{Parser, Subcommand};
use std::fmt::Debug;

// See https://github.com/clap-rs/clap/issues/975#issuecomment-1426424232
// for the issue of having a default subcommand.

#[derive(Parser, Debug)]
#[command(version)]
#[command(propagate_version = true)]
#[command(args_conflicts_with_subcommands = true)]
#[clap(verbatim_doc_comment)]
/// Format filenames according to a chosen naming convention.
///
/// For each file/path (of any kind) given as argument, change the filename
/// (i.e. the base of the path) according to the selected naming convention.
///
/// WARNING! This program is dangerous.
/// Changing filenames is error prone and may cause undesired consequences
/// (some files are expected to have the name they have and not something else!).
/// fmtna's solves these problems by:
///     1. Asking you what to do when conflicts happen (the program
///        wants to change a path to an already existing path).
///     2. Backing up the filename changes and allowing you to revert
///        the changes partially or completely.
///     3. Giving you ways to exclude some filenames from formatting.
/// Still, fmtna can't stop you from shooting yourself in the foot.
/// It can go as far as corrupting your system.
pub struct Cli {
    #[command(subcommand)]
    /// The subcommand.
    pub command: Option<Command>,

    #[clap(flatten)]
    /// Arguments of the default command (fmtna), i.e. when no subcommand is used.
    pub args: DefaultArgs,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
#[clap(verbatim_doc_comment)]
/// Subcommands.
pub enum Command {
    #[allow(missing_docs)]
    Exclude(ExcludeCli),
    #[allow(missing_docs)]
    Revert(RevertCli),
}
