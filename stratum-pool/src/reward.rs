use crate::database::Database;
use anyhow::Result;
use crate::config::PoolConfig;
use log::info;
use std::sync::Arc;

pub struct RewardDistributor {
    db: Arc<Database>,
    config: PoolConfig,
}

impl RewardDistributor {
    pub fn new(db: Arc<Database>, config: PoolConfig) -> Self {
        Self { db, config }
    }

    pub async fn run(&self) {
        let mut interval = tokio::time::interval(
            std::time::Duration::from_secs(self.config.reward.distribution_interval)
        );

        loop {
            interval.tick().await;

            if let Err(e) = self.distribute_rewards().await {
                log::error!("Reward distribution error: {}", e);
            }
        }
    }

    async fn distribute_rewards(&self) -> Result<()> {
        info!("Distributing rewards...");

        // Get unpaid rewards
        let rewards = crate::database::get_unpaid_rewards(self.db.pool()).await?;

        for (reward_id, amount, _address) in rewards {
            // Mark as paid
            crate::database::mark_reward_paid(self.db.pool(), reward_id).await?;
            info!("Paid reward: {} TRK", amount as f64 / 100_000_000.0);
        }

        Ok(())
    }
}
