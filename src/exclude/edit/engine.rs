use anyhow::{anyhow, Context};

use super::cli::EditCli;
use super::data::Data;
use crate::cfg::Cfg;
use crate::engine::Engine;
use crate::utils::get_exclude_file_path;
use std::process::Command;

pub fn get_engine(cli: EditCli, cfg: Cfg) -> anyhow::Result<Box<dyn Engine>> {
    Ok(Box::new(EditEngine::new(cli, cfg)?))
}

struct EditEngine {
    data: Data,
}

impl EditEngine {
    pub fn new(cli: EditCli, cfg: Cfg) -> anyhow::Result<Self> {
        let data = Data::new(cli, cfg)?;
        Ok(Self { data })
    }
}

impl Engine for EditEngine {
    fn run(&mut self) -> anyhow::Result<()> {
        let exclude_file_path = get_exclude_file_path()?;

        let status = Command::new(self.data.editor.clone())
            .arg(exclude_file_path.clone())
            .status()
            .with_context(|| {
                format!(
                    "Failed to run '{} {}'.",
                    self.data.editor,
                    exclude_file_path.to_string_lossy()
                )
            })?;

        if !status.success() {
            return Err(anyhow!(
                "Command '{} {}' failed. Exited with status {}.",
                self.data.editor,
                exclude_file_path.to_string_lossy(),
                status,
            ));
        }

        Ok(())
    }
}
