use super::cli::EditCli;
use crate::cfg::Cfg;

#[derive(Debug, PartialEq, Eq)]
pub struct Data {
    pub editor: String,
}

impl Data {
    pub fn new(cli: EditCli, cfg: Cfg) -> anyhow::Result<Self> {
        Ok(Data {
            editor: cli.editor.unwrap_or(cfg.editor),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::naming_conventions::NamingConvention;

    #[derive(Debug)]
    struct TestCase {
        cli: EditCli,
        cfg: Cfg,
        data: Data,
    }

    #[test]
    fn cli_takes_precedence_on_config() {
        let test_cases = vec![
            // Cli takes precedence
            TestCase {
                cli: EditCli {
                    editor: Some(String::from("nvim")),
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
                    editor: String::from("nvim"),
                },
            },
            // Defaults to config otherwise
            TestCase {
                cli: EditCli { editor: None },
                cfg: Cfg {
                    naming_convention: NamingConvention::SnakeCase,
                    recursive: false,
                    keep_dots: false,
                    keep_special_chars: false,
                    keep_unicode: false,
                    editor: String::from("emacs"),
                },
                data: Data {
                    editor: String::from("emacs"),
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
