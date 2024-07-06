use clap::Args;

#[derive(Args, Clone, Debug, PartialEq, Eq)]
#[clap(verbatim_doc_comment)]
/// Open exclude.txt for edition.
pub struct EditCli {
    #[clap(verbatim_doc_comment)]
    /// The editor with which to open exclude.txt.
    ///
    /// If not specified, the value for editor in the config file
    /// is used (it defaults to "vi").
    pub editor: Option<String>,
}
