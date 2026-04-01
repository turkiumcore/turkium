use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_server_host")]
    pub server_host: String,

    #[serde(default = "default_server_port")]
    pub server_port: u16,

    #[serde(default = "default_environment")]
    pub environment: String,

    #[serde(default = "default_turkium_node_host")]
    pub turkium_node_host: String,

    #[serde(default = "default_turkium_node_port")]
    pub turkium_node_port: u16,

    #[serde(default = "default_turkium_node_timeout")]
    pub turkium_node_timeout_secs: u64,

    #[serde(default = "default_cors_origins")]
    #[allow(dead_code)]
    pub cors_allowed_origins: String,

    #[serde(default = "default_cache_ttl")]
    pub cache_ttl_secs: u64,

    #[serde(default = "default_cache_capacity")]
    pub cache_max_capacity: u64,

    #[serde(default = "default_rate_limit_requests")]
    #[allow(dead_code)]
    pub rate_limit_requests: u32,

    #[serde(default = "default_rate_limit_window")]
    #[allow(dead_code)]
    pub rate_limit_window_secs: u64,
}

fn default_server_host() -> String {
    "0.0.0.0".to_string()
}

fn default_server_port() -> u16 {
    3001
}

fn default_environment() -> String {
    "development".to_string()
}

fn default_turkium_node_host() -> String {
    "localhost".to_string()
}

fn default_turkium_node_port() -> u16 {
    5200
}

fn default_turkium_node_timeout() -> u64 {
    10
}

fn default_cors_origins() -> String {
    "*".to_string()
}

fn default_cache_ttl() -> u64 {
    30
}

fn default_cache_capacity() -> u64 {
    10000
}

fn default_rate_limit_requests() -> u32 {
    1000
}

fn default_rate_limit_window() -> u64 {
    60
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let config = envy::from_env::<Config>()
            .map_err(|e| anyhow::anyhow!("Failed to load config from env: {}", e))?;
        Ok(config)
    }
}
