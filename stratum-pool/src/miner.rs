use crate::database::Database;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::net::TcpStream;

pub struct MinerInfo {
    pub id: i64,
    pub address: String,
    pub difficulty: f64,
    pub shares: u64,
}

pub struct MinerManager {
    db: Arc<Database>,
    miners: DashMap<i64, MinerInfo>,
}

impl MinerManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            miners: DashMap::new(),
        }
    }

    pub async fn add_miner(
        &self,
        miner_id: i64,
        address: String,
        _socket: &mut TcpStream,
    ) -> anyhow::Result<()> {
        let info = MinerInfo {
            id: miner_id,
            address,
            difficulty: 1.0,
            shares: 0,
        };

        self.miners.insert(miner_id, info);
        Ok(())
    }

    pub fn get_miner(&self, miner_id: i64) -> Option<MinerInfo> {
        self.miners.get(&miner_id).map(|m| MinerInfo {
            id: m.id,
            address: m.address.clone(),
            difficulty: m.difficulty,
            shares: m.shares,
        })
    }

    pub async fn update_difficulty(
        &self,
        miner_id: i64,
        difficulty: f64,
    ) -> anyhow::Result<()> {
        if let Some(mut miner) = self.miners.get_mut(&miner_id) {
            miner.difficulty = difficulty;
        }

        crate::database::update_miner_difficulty(self.db.pool(), miner_id, difficulty).await?;
        Ok(())
    }

    pub fn get_all_miners(&self) -> Vec<MinerInfo> {
        self.miners
            .iter()
            .map(|entry| MinerInfo {
                id: entry.value().id,
                address: entry.value().address.clone(),
                difficulty: entry.value().difficulty,
                shares: entry.value().shares,
            })
            .collect()
    }
}
