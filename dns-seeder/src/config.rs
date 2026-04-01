use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub hostname: String,
    pub peer_port: u16,
    pub dns_port: u16,
    pub dns_listen: String,
    pub bootstrap_peer: Option<String>,
    pub network: String,
    pub min_peers: usize,
    pub max_peers: usize,
    pub crawl_interval: u64,
    pub peer_timeout: u64,
}

impl Config {
    pub fn get_default_peer_port(&self) -> u16 {
        match self.network.as_str() {
            "mainnet" => 5206,
            "testnet" => 5207,
            "devnet" => 5208,
            _ => 5206,
        }
    }

    pub fn get_default_rpc_port(&self) -> u16 {
        match self.network.as_str() {
            "mainnet" => 5200,
            "testnet" => 5201,
            "devnet" => 5202,
            _ => 5200,
        }
    }
}
