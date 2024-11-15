//! Paths used throughout the program.

use clap::crate_name;
use directories::ProjectDirs;
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    /// Absolute path to the exclude file.
    pub static ref EXCLUDE_FILE_PATH: PathBuf = {
        let mut exclude_file_path = ProjectDirs::from("", crate_name!(), crate_name!())
            .unwrap()
            .config_local_dir()
            .to_path_buf();
        exclude_file_path.push("exclude.txt");
        exclude_file_path
    };
    /// Absolute path to the history directory.
    pub static ref HISTORY_DIR_PATH: PathBuf = {
        let mut history_dir_path = ProjectDirs::from("", crate_name!(), crate_name!())
            .unwrap()
            .config_local_dir()
            .to_path_buf();
        history_dir_path.push("history");
        history_dir_path
    };
    /// Absolute path to the history directory.
    pub static ref BACKUP_DIR_PATH: PathBuf = {
        let mut backup_dir_path = ProjectDirs::from("", crate_name!(), crate_name!())
            .unwrap()
            .config_local_dir()
            .to_path_buf();
        backup_dir_path.push("backups");
        backup_dir_path
    };
}

#[cfg(test)]
pub mod tests {
    use lazy_static::lazy_static;
    use std::path::PathBuf;

    lazy_static! {
        /// Absolute path to the "temporary" directory, where test files live.
        pub static ref TMP_DIR_PATH: PathBuf = {
            let mut tmp_dir = std::env::current_dir().unwrap();
            tmp_dir.push(".tmp");
            tmp_dir
        };
    }
}
