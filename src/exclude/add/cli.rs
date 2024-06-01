use clap::Args;

#[derive(Args, Clone, Debug, PartialEq, Eq)]
#[clap(verbatim_doc_comment)]
/// Add a pattern to exclude.txt.
pub struct AddCli {
    #[clap(verbatim_doc_comment)]
    /// A pattern to add to exclude.txt.
    ///
    /// If the pattern is already in exclude.txt,
    /// nothing will happen and you will be warned about it.
    pub pattern: String,
}
