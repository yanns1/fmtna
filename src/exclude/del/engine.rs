use super::cli::DelCli;
use super::data::Data;
use crate::cfg::Cfg;
use crate::engine::Engine;

pub fn get_engine(cli: DelCli, cfg: Cfg) -> anyhow::Result<Box<dyn Engine>> {
    Ok(Box::new(DelEngine::new(cli, cfg)?))
}

struct DelEngine {
    _data: Data,
}

impl DelEngine {
    pub fn new(cli: DelCli, cfg: Cfg) -> anyhow::Result<Self> {
        let data = Data::new(cli, cfg)?;
        Ok(Self { _data: data })
    }
}

impl Engine for DelEngine {
    fn run(&self) -> anyhow::Result<()> {
        todo!()
    }
}
