use super::cli::DelCli;
use crate::cfg::Cfg;

/// An aggregation of configurations coming from the [default::Cli](crate::default::cli::DefaultArgs) and the configuration file ([`Cfg`]).
/// A configuration coming from the CLI always takes precedence.
/// A configuration coming from the configuration file is applied only when the equivalent is not
/// specified at the CLI level.
#[derive(Debug)]
pub struct Data {
    pub exclude_pattern: String,
}

impl Data {
    pub fn new(cli: DelCli, cfg: Cfg) -> anyhow::Result<Self> {
        let _ = cfg;

        Ok(Data {
            exclude_pattern: cli.pattern,
        })
    }
}
