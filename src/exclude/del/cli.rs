use clap::Args;

#[derive(Args, Clone, Debug, PartialEq, Eq)]
#[clap(verbatim_doc_comment)]
/// Delete a pattern from exclude.txt.
pub struct DelCli {
    #[clap(verbatim_doc_comment)]
    /// The pattern to delete from exclude.txt.
    ///
    /// If the pattern is not found in exclude.txt,
    /// nothing will happen and you will be warned about it.
    /// Furthermore, the closest pattern found in the file
    /// will be proposed for deletion as a guess for
    /// what you really wanted to delete.
    pub pattern: String,
}
