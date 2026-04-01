use crate::database::Database;
use dashmap::DashMap;
use std::sync::Arc;

pub struct MinerInfo;

pub struct MinerManager {
    miners: DashMap<i64, MinerInfo>,
}

impl MinerManager {
    pub fn new(_db: Arc<Database>) -> Self {
        Self {
            miners: DashMap::new(),
        }
    }

    pub async fn add_miner(
        &self,
        miner_id: i64,
        _address: String,
    ) -> anyhow::Result<()> {
        self.miners.insert(miner_id, MinerInfo);
        Ok(())
    }
}
