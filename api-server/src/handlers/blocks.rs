use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

use crate::cache::ResponseCache;
use crate::error::ApiResult;
use crate::rpc_client::TurkiumRpcClient;

pub async fn get_block(
    State((rpc_client, cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
    Path(hash): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let cache_key = format!("block:{}", hash);

    if let Some(cached) = cache.get(&cache_key).await {
        return Ok(Json(cached));
    }

    let block_data = rpc_client.get_block(&hash).await?;
    let block: serde_json::Value = serde_json::from_str(&block_data)
        .unwrap_or_else(|_| serde_json::json!({"error": "Invalid block data"}));

    cache.set(cache_key, block.clone()).await;

    Ok(Json(block))
}
