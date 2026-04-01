use moka::future::Cache;
use serde_json::Value;
use std::time::Duration;

pub struct ResponseCache {
    cache: Cache<String, Value>,
}

impl ResponseCache {
    pub fn new(ttl_secs: u64, max_capacity: u64) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(ttl_secs))
            .build();

        ResponseCache {
            cache,
        }
    }

    pub async fn get(&self, key: &str) -> Option<Value> {
        self.cache.get(key).await
    }

    pub async fn set(&self, key: String, value: Value) {
        self.cache.insert(key, value).await;
    }
}
