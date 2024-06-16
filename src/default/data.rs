use std::path::PathBuf;

use regex::Regex;

use super::cli::DefaultArgs;
use crate::cfg::Cfg;
use crate::exclude::get_exclude_file_path;
use crate::naming_conventions::NamingConvention;
use anyhow::anyhow;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
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

    /// Same as [Cli::keep_unicode](crate::cli::Cli::keep_unicode)
    pub keep_unicode: bool,

    pub exclude_regexes: Vec<Regex>,
}

impl Data {
    pub fn new(cli: DefaultArgs, cfg: Cfg) -> anyhow::Result<Self> {
        let naming_convention = cli.naming_convention.unwrap_or(cfg.naming_convention);
        let recursive = cli.recursive || cfg.recursive;
        let keep_dots = cli.keep_dots || cfg.keep_dots;
        let keep_special_chars = cli.keep_special_chars || cfg.keep_special_chars;
        let keep_unicode = cli.keep_unicode || cfg.keep_unicode;

        // NOTE: We store regexes into a vec, but the exclude file can be so big
        // that the program's memory will not suffice.
        // Furthermore, large number of patterns may negatively affect performance,
        // but not sure if it will ever by a practical concern, so keep the simple
        // way of doing things for now.
        let mut exclude_regexes: Vec<Regex> = vec![];
        let exclude_file_path = get_exclude_file_path()?;
        if exclude_file_path.exists() {
            let file = File::open(exclude_file_path.clone())?;
            let reader = BufReader::new(file);
            for (line_no, line) in reader.lines().enumerate() {
                let line = line?;

                if line.is_empty() || line.starts_with("//") {
                    continue;
                }

                match Regex::new(&line) {
                    Ok(exclude_re) => {
                        exclude_regexes.push(exclude_re);
                    }
                    Err(_) => {
                        return Err(anyhow!(
                            "Exclude pattern {} is invalid (in {}, line {}).",
                            line,
                            exclude_file_path.to_string_lossy(),
                            line_no
                        ));
                    }
                }
            }
        }

        Ok(Data {
            files: cli.files,
            naming_convention,
            recursive,
            keep_dots,
            keep_special_chars,
            keep_unicode,
            exclude_regexes,
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
                    keep_unicode: true,
                },
                cfg: Cfg {
                    naming_convention: NamingConvention::SnakeCase,
                    recursive: false,
                    keep_dots: false,
                    keep_special_chars: false,
                    keep_unicode: false,
                    editor: String::from("vi"),
                },
                data: Data {
                    files: vec![],
                    naming_convention: NamingConvention::CamelCase,
                    recursive: true,
                    keep_dots: true,
                    keep_special_chars: true,
                    keep_unicode: true,
                    exclude_regexes: vec![],
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
                    keep_unicode: false,
                },
                cfg: Cfg {
                    naming_convention: NamingConvention::SnakeCase,
                    recursive: true,
                    keep_dots: false,
                    keep_special_chars: true,
                    keep_unicode: true,
                    editor: String::from("vi"),
                },
                data: Data {
                    files: vec![],
                    naming_convention: NamingConvention::SnakeCase,
                    recursive: true,
                    keep_dots: false,
                    keep_special_chars: true,
                    keep_unicode: true,
                    exclude_regexes: vec![],
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
                    keep_unicode: true,
                },
                cfg: Cfg {
                    naming_convention: NamingConvention::SnakeCase,
                    recursive: false,
                    keep_dots: false,
                    keep_special_chars: true,
                    keep_unicode: false,
                    editor: String::from("vi"),
                },
                data: Data {
                    files: vec![],
                    naming_convention: NamingConvention::CamelCase,
                    recursive: true,
                    keep_dots: false,
                    keep_special_chars: true,
                    keep_unicode: true,
                    exclude_regexes: vec![],
                },
            },
        ];

        for test_case in test_cases {
            let data = Data::new(test_case.cli, test_case.cfg)
                .expect("Data::new should have succeed. There must be an error in the test case.");

            assert_eq!(
                data.files, test_case.data.files,
                "Expected {:?}, but got {:?}",
                data.files, test_case.data.files
            );
            assert_eq!(
                data.naming_convention, test_case.data.naming_convention,
                "Expected {:?}, but got {:?}",
                data.naming_convention, test_case.data.naming_convention
            );
            assert_eq!(
                data.recursive, test_case.data.recursive,
                "Expected {:?}, but got {:?}",
                data.recursive, test_case.data.recursive
            );
            assert_eq!(
                data.keep_dots, test_case.data.keep_dots,
                "Expected {:?}, but got {:?}",
                data.keep_dots, test_case.data.keep_dots
            );
            assert_eq!(
                data.keep_special_chars, test_case.data.keep_special_chars,
                "Expected {:?}, but got {:?}",
                data.keep_special_chars, test_case.data.keep_special_chars
            );
            assert_eq!(
                data.keep_unicode, test_case.data.keep_unicode,
                "Expected {:?}, but got {:?}",
                data.keep_unicode, test_case.data.keep_unicode
            );
        }
    }
}
