use super::RevertCli;
use crate::cfg::Cfg;
use crate::engine::Engine;
use crate::revert::data::Data;

pub fn get_engine(cli: RevertCli, cfg: Cfg) -> anyhow::Result<Box<dyn Engine>> {
    Ok(Box::new(RevertEngine::new(cli, cfg)?))
}

struct RevertEngine {
    _data: Data,
}

impl RevertEngine {
    pub fn new(cli: RevertCli, cfg: Cfg) -> anyhow::Result<Self> {
        let data = Data::new(cli, cfg)?;
        Ok(Self { _data: data })
    }
}

impl Engine for RevertEngine {
    fn run(&self) -> anyhow::Result<()> {
        todo!()
    }
}
