use super::cli::EditCli;
use super::data::Data;
use crate::cfg::Cfg;
use crate::engine::Engine;

pub fn get_engine(cli: EditCli, cfg: Cfg) -> anyhow::Result<Box<dyn Engine>> {
    Ok(Box::new(EditEngine::new(cli, cfg)?))
}

struct EditEngine {
    _data: Data,
}

impl EditEngine {
    pub fn new(cli: EditCli, cfg: Cfg) -> anyhow::Result<Self> {
        let data = Data::new(cli, cfg)?;
        Ok(Self { _data: data })
    }
}

impl Engine for EditEngine {
    fn run(&self) -> anyhow::Result<()> {
        todo!()
    }
}
