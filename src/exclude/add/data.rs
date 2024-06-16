use anyhow::Context;

use super::cli::AddCli;
use crate::cfg::Cfg;
use regex::Regex;

#[derive(Debug)]
pub struct Data {
    pub _exclude_re: Regex,
}

impl Data {
    pub fn new(cli: AddCli, cfg: Cfg) -> anyhow::Result<Self> {
        let _ = cfg;

        let exclude_re =
            Regex::new(&cli.pattern).with_context(|| "The pattern given is not valid.")?;

        Ok(Data {
            _exclude_re: exclude_re,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::naming_conventions::NamingConvention;

    #[derive(Debug)]
    struct TestCase {
        cli: AddCli,
        cfg: Cfg,
    }

    #[test]
    fn data_instantiation_succeeds_if_valid_patterns() {
        let valid_patterns = vec![
            "Makefile",
            "README.*",
            "LICENSE.*",
            r".*\.js",
            r".*\.ts",
            r".*\.jsx",
            r".*\.tsx",
            r".*\.hs",
            r".*\.rs",
            r".*\.py",
            r".*\.c",
            r".*\.cpp",
            r".*\.cxx",
            r".*\.h",
            r".*\.hpp",
            r".*\.hxx",
            r".*\.html",
            r".*\.css",
        ];
        let mut test_cases: Vec<TestCase> = vec![];
        for pattern in valid_patterns {
            test_cases.push(TestCase {
                cli: AddCli {
                    pattern: String::from(pattern),
                },
                cfg: Cfg {
                    naming_convention: NamingConvention::SnakeCase,
                    recursive: false,
                    keep_dots: false,
                    keep_special_chars: false,
                    keep_unicode: false,
                    editor: String::from("vi"),
                },
            })
        }

        for test_case in test_cases {
            Data::new(test_case.cli, test_case.cfg)
                .expect("Data::new should have succeed. There must be an error in the test case, or the pattern is indeed invalid.");
        }
    }

    #[test]
    fn data_instantiation_fails_if_invalid_patterns() {
        let invalid_patterns = vec!["***", "(((", "[[["];
        let mut test_cases: Vec<TestCase> = vec![];
        for pattern in invalid_patterns {
            test_cases.push(TestCase {
                cli: AddCli {
                    pattern: String::from(pattern),
                },
                cfg: Cfg {
                    naming_convention: NamingConvention::SnakeCase,
                    recursive: false,
                    keep_dots: false,
                    keep_special_chars: false,
                    keep_unicode: false,
                    editor: String::from("vi"),
                },
            })
        }

        for test_case in test_cases {
            assert!(
                Data::new(test_case.cli, test_case.cfg).is_err(),
                "Expected Data:new to error."
            )
        }
    }
}
