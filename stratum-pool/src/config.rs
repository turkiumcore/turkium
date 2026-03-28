use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub pool: PoolConfig,
    pub blockchain: BlockchainConfig,
    pub database: DatabaseConfig,
    pub reward: RewardConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    pub listen_address: String,
    pub difficulty: f64,
    pub min_difficulty: f64,
    pub max_difficulty: f64,
    pub share_multiplier: f64,
    pub pool_fee: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    pub grpc_address: String,
    pub template_update_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardConfig {
    pub distribution_interval: u64,
    pub min_payout: u64,
}

impl Config {
    pub async fn from_file(path: &str) -> Result<Self> {
        // Try to load from file, if not found create default
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                log::info!("Config file not found, creating default: {}", path);
                let default = Self::default();
                let toml = toml::to_string_pretty(&default)?;
                fs::write(path, &toml)?;
                toml
            }
        };

        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pool: PoolConfig {
                listen_address: "0.0.0.0:3333".to_string(),
                difficulty: 1.0,
                min_difficulty: 0.1,
                max_difficulty: 1000.0,
                share_multiplier: 1.0,
                pool_fee: 0.01, // 1%
            },
            blockchain: BlockchainConfig {
                grpc_address: "grpc://188.132.197.20:5200".to_string(),
                template_update_interval: 10,
            },
            database: DatabaseConfig {
                path: "pool.db".to_string(),
            },
            reward: RewardConfig {
                distribution_interval: 86400, // 24 hours
                min_payout: 1000000,           // 0.01 TRK
            },
        }
    }
}
