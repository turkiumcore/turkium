use crate::config::PoolConfig;
use crate::database::Database;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ShareProcessor {
    db: Arc<Database>,
    config: PoolConfig,
}

impl ShareProcessor {
    pub fn new(
        db: Arc<Database>,
        _blockchain: Arc<Mutex<crate::blockchain::BlockchainClient>>,
        config: PoolConfig,
    ) -> Self {
        Self {
            db,
            config,
        }
    }

    pub async fn process_share(
        &self,
        miner_id: i64,
        job_id: &str,
        _nonce: &str,
        _result: &str,
    ) -> Result<(bool, f64)> {
        // Get current difficulty for miner
        let (_, total_diff) = crate::database::get_miner_stats(self.db.pool(), miner_id).await?;

        // Difficulty for this share
        let share_difficulty = self.config.difficulty;

        // Save share to database
        crate::database::save_share(
            self.db.pool(),
            miner_id,
            job_id,
            share_difficulty,
            false, // is_block
        )
        .await?;

        // Adjust difficulty based on share rate
        let new_difficulty = self.calculate_difficulty(total_diff);
        if (new_difficulty - self.config.difficulty).abs() > 0.01 {
            crate::database::update_miner_difficulty(self.db.pool(), miner_id, new_difficulty)
                .await?;
        }

        Ok((false, share_difficulty))
    }

    fn calculate_difficulty(&self, total_difficulty: f64) -> f64 {
        // Simple difficulty adjustment
        // Target: 1 share per 30 seconds
        let target_shares_per_hour = 120.0;
        let current_rate = total_difficulty / 3600.0; // Assume 1 hour window

        if current_rate > 0.0 {
            let new_diff = self.config.difficulty * (target_shares_per_hour / current_rate);
            new_diff
                .max(self.config.min_difficulty)
                .min(self.config.max_difficulty)
        } else {
            self.config.difficulty
        }
    }
}
