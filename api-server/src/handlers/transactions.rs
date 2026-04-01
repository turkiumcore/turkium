use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use std::sync::Arc;
use tracing::warn;

use crate::cache::ResponseCache;
use crate::error::ApiResult;
use crate::models::*;
use crate::rpc_client::TurkiumRpcClient;

pub async fn get_transaction(
    State((rpc_client, cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
    Path(hash): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let cache_key = format!("tx:{}", hash);

    if let Some(cached) = cache.get(&cache_key).await {
        return Ok(Json(cached));
    }

    let tx_data = rpc_client.get_block(&hash).await?;
    let tx_json: serde_json::Value = serde_json::from_str(&tx_data)
        .unwrap_or_else(|_| serde_json::json!({"error": "Invalid transaction data"}));
    
    cache.set(cache_key, tx_json.clone()).await;

    Ok(Json(tx_json))
}

pub async fn get_transaction_count(
    State((rpc_client, _cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
) -> ApiResult<Json<TransactionCount>> {
    // Get mempool entries count as current transaction count
    let mempool_entries = rpc_client.get_mempool_entries().await?;
    
    Ok(Json(TransactionCount {
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        date_time: Utc::now(),
        coinbase: 0,
        regular: mempool_entries.len() as u64,
    }))
}

pub async fn search_transactions(
    State((rpc_client, _cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
    Json(request): Json<SearchTransactionsRequest>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let mut transactions = Vec::new();

    for tx_id in request.transaction_ids {
        match rpc_client.get_block(&tx_id).await {
            Ok(tx_data) => {
                if let Ok(tx_json) = serde_json::from_str::<serde_json::Value>(&tx_data) {
                    transactions.push(tx_json);
                }
            }
            Err(e) => {
                warn!("Failed to fetch transaction {}: {}", tx_id, e);
            }
        }
    }

    Ok(Json(transactions))
}
