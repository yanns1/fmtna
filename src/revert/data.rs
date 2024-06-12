use crate::cfg::Cfg;
use crate::revert::cli::RevertCli;
use anyhow::anyhow;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct Data {
    pub backup_file: PathBuf,
}

impl Data {
    pub fn new(cli: RevertCli, cfg: Cfg) -> anyhow::Result<Self> {
        let _ = cfg;

        if !cli.backup_file.exists() {
            return Err(anyhow!(format!("{:?} does not exist.", cli.backup_file)));
        }

        Ok(Data {
            backup_file: cli.backup_file,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::naming_conventions::NamingConvention;
    use crate::utils::test::get_tmp_dir;
    use serial_test::serial;
    use std::fs;

    #[derive(Debug)]
    struct TestCase {
        cli: RevertCli,
        cfg: Cfg,
        data: Data,
    }

    fn mk_backup_file() -> PathBuf {
        let tmp_dir = get_tmp_dir();
        if !tmp_dir.exists() {
            if let Err(err) = fs::create_dir(&tmp_dir) {
                panic!("{:?}", err);
            }
        }

        let mut backup_file = tmp_dir.clone();
        backup_file.push("backup_file");
        let lines = vec![String::from("")];
        if let Err(err) = fs::write(&backup_file, lines.join("\n")) {
            panic!("{:?}", err);
        }

        return backup_file;
    }

    #[serial]
    #[test]
    fn data_instantiation_succeeds_if_valid_backup_file() {
        let backup_file = mk_backup_file();

        let test_cases = vec![
            // Cli takes precedence
            TestCase {
                cli: RevertCli {
                    backup_file: backup_file.clone(),
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
                    backup_file: backup_file.clone(),
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

    #[test]
    fn data_instantiation_fails_if_invalid_backup_file() {
        let mut backup_file = get_tmp_dir();
        backup_file.push("inexistant_backup_file");

        let cli = RevertCli {
            backup_file: backup_file.clone(),
        };
        let cfg = Cfg {
            naming_convention: NamingConvention::SnakeCase,
            recursive: false,
            keep_dots: false,
            keep_special_chars: false,
            keep_unicode: false,
            editor: String::from("vi"),
        };

        assert!(Data::new(cli, cfg).is_err(), "Expected Data::new to fail.",);
    }
}
