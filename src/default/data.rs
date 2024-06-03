use std::path::PathBuf;

use super::cli::DefaultArgs;
use crate::cfg::Cfg;
use crate::naming_conventions::NamingConvention;

#[derive(Debug, PartialEq, Eq)]
pub struct Data {
    /// Same as [Cli::files](crate::cli::Cli::files)
    pub files: Vec<PathBuf>,

    /// Same as [Cli::naming_convention](crate::cli::Cli::naming_convention)
    pub naming_convention: NamingConvention,

    /// Same as [Cli::recursive](crate::cli::Cli::recursive)
    pub recursive: bool,

    /// Same as [Cli::keep_dots](crate::cli::Cli::keep_dots)
    pub keep_dots: bool,

    /// Same as [Cli::keep_special_chars](crate::cli::Cli::keep_special_chars)
    pub keep_special_chars: bool,
}

impl Data {
    pub fn new(cli: DefaultArgs, cfg: Cfg) -> anyhow::Result<Self> {
        let naming_convention = cli.naming_convention.unwrap_or(cfg.naming_convention);
        let recursive = cli.recursive || cfg.recursive;
        let keep_dots = cli.keep_dots || cfg.keep_dots;
        let keep_special_chars = cli.keep_special_chars || cfg.keep_special_chars;

        Ok(Data {
            files: cli.files,
            naming_convention,
            recursive,
            keep_dots,
            keep_special_chars,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCase {
        cli: DefaultArgs,
        cfg: Cfg,
        data: Data,
    }

    #[test]
    fn cli_takes_precedence_on_config() {
        let test_cases = vec![
            // Cli takes precedence
            TestCase {
                cli: DefaultArgs {
                    files: vec![],
                    naming_convention: Some(NamingConvention::CamelCase),
                    recursive: true,
                    keep_dots: true,
                    keep_special_chars: true,
                },
                cfg: Cfg {
                    naming_convention: NamingConvention::SnakeCase,
                    recursive: false,
                    keep_dots: false,
                    keep_special_chars: false,
                    editor: String::from("vi"),
                },
                data: Data {
                    files: vec![],
                    naming_convention: NamingConvention::CamelCase,
                    recursive: true,
                    keep_dots: true,
                    keep_special_chars: true,
                },
            },
            // When option not defined via Cli, backup to Cfg
            TestCase {
                cli: DefaultArgs {
                    files: vec![],
                    naming_convention: None,
                    recursive: false,
                    keep_dots: false,
                    keep_special_chars: false,
                },
                cfg: Cfg {
                    naming_convention: NamingConvention::SnakeCase,
                    recursive: true,
                    keep_dots: false,
                    keep_special_chars: true,
                    editor: String::from("vi"),
                },
                data: Data {
                    files: vec![],
                    naming_convention: NamingConvention::SnakeCase,
                    recursive: true,
                    keep_dots: false,
                    keep_special_chars: true,
                },
            },
            // A mix of options coming from Cli and others from Cfg
            TestCase {
                cli: DefaultArgs {
                    files: vec![],
                    naming_convention: Some(NamingConvention::CamelCase),
                    recursive: true,
                    keep_dots: false,
                    keep_special_chars: false,
                },
                cfg: Cfg {
                    naming_convention: NamingConvention::SnakeCase,
                    recursive: false,
                    keep_dots: false,
                    keep_special_chars: true,
                    editor: String::from("vi"),
                },
                data: Data {
                    files: vec![],
                    naming_convention: NamingConvention::CamelCase,
                    recursive: true,
                    keep_dots: false,
                    keep_special_chars: true,
                },
            },
        ];

        for test_case in test_cases {
            let data = Data::new(test_case.cli, test_case.cfg)
                .expect("Data::new should have succeed. There must be an error in the test case.");
            assert_eq!(
                data, test_case.data,
                "Expected {:?}, but got {:?}",
                test_case.data, data
            );
        }
    }
}
