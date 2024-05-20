use std::path::PathBuf;

use anyhow::anyhow;

use crate::cfg::Cfg;
use crate::cli::Cli;
use crate::cli::Command;
use crate::cli::ExcludeCommand;
use crate::cli::NamingConvention;

/// An aggregation of configurations coming from the CLI ([`Cli`]) and the configuration file ([`Cfg`]).
/// A configuration coming from the CLI always takes precedence.
/// A configuration coming from the configuration file is applied only when the equivalent is not
/// specified at the CLI level.
#[derive(Debug, PartialEq, Eq)]
pub struct Params {
    /// Same as [Cli::command](crate::cli::Cli::command)
    pub command: Option<Command>,

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

    /// Same as [ExcludeCommand::Edit::editor](crate::cli::ExcludeCommand::Edit::editor)
    pub editor: Option<String>,
}

impl Params {
    pub fn new(cli: Cli, cfg: Cfg) -> anyhow::Result<Self> {
        // Check that all paths are valid
        for file in &cli.files {
            if !file.exists() {
                return Err(anyhow!(format!("{:?} does not exist.", file)));
            }
        }

        let naming_convention = cli.naming_convention.unwrap_or(cfg.naming_convention);
        let recursive = cli.recursive || cfg.recursive;
        let keep_dots = cli.keep_dots || cfg.keep_dots;
        let keep_special_chars = cli.keep_special_chars || cfg.keep_special_chars;
        let mut editor: Option<String> = None;
        if let Some(Command::Exclude {
            command: ExcludeCommand::Edit { editor: cli_editor },
        }) = cli.command.clone()
        {
            editor = cli_editor.or(Some(cfg.editor))
        }

        Ok(Params {
            files: cli.files,
            command: cli.command,
            naming_convention,
            recursive,
            keep_dots,
            keep_special_chars,
            editor,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCase {
        cli: Cli,
        cfg: Cfg,
        params: Params,
    }

    #[test]
    fn cli_takes_precedence_on_config() {
        let test_cases = vec![
            // Cli takes precedence
            TestCase {
                cli: Cli {
                    command: Some(Command::Exclude {
                        command: ExcludeCommand::Edit {
                            editor: Some(String::from("nvim")),
                        },
                    }),
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
                params: Params {
                    command: Some(Command::Exclude {
                        command: ExcludeCommand::Edit {
                            editor: Some(String::from("nvim")),
                        },
                    }),
                    files: vec![],
                    naming_convention: NamingConvention::CamelCase,
                    recursive: true,
                    keep_dots: true,
                    keep_special_chars: true,
                    editor: Some(String::from("nvim")),
                },
            },
            // When option not defined via Cli, backup to Cfg
            TestCase {
                cli: Cli {
                    command: None,
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
                params: Params {
                    command: None,
                    files: vec![],
                    naming_convention: NamingConvention::SnakeCase,
                    recursive: true,
                    keep_dots: false,
                    keep_special_chars: true,
                    editor: None,
                },
            },
            // A mix of options coming from Cli and others from Cfg
            TestCase {
                cli: Cli {
                    command: None,
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
                params: Params {
                    command: None,
                    files: vec![],
                    naming_convention: NamingConvention::CamelCase,
                    recursive: true,
                    keep_dots: false,
                    keep_special_chars: true,
                    editor: None,
                },
            },
        ];

        for test_case in test_cases {
            let params = Params::new(test_case.cli, test_case.cfg).expect(
                "Params::new should have succeed. There must be an error in the test case.",
            );
            assert_eq!(
                params, test_case.params,
                "Expected {:?}, but got {:?}",
                test_case.params, params
            );
        }
    }
}
