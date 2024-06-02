use super::cli::AddCli;
use super::data::Data;
use crate::cfg::Cfg;
use crate::engine::Engine;

pub fn get_engine(cli: AddCli, cfg: Cfg) -> anyhow::Result<Box<dyn Engine>> {
    Ok(Box::new(AddEngine::new(cli, cfg)?))
}

struct AddEngine {
    _data: Data,
}

impl AddEngine {
    pub fn new(cli: AddCli, cfg: Cfg) -> anyhow::Result<Self> {
        let data = Data::new(cli, cfg)?;
        Ok(Self { _data: data })
    }
}

impl Engine for AddEngine {
    fn run(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}
