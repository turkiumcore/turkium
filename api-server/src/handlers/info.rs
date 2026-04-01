use axum::{extract::State, Json};
use std::sync::Arc;

use crate::cache::ResponseCache;
use crate::error::ApiResult;
use crate::rpc_client::TurkiumRpcClient;

// ============================================
// BlockDAG Info
// ============================================

pub async fn get_blockdag_info(
    State((rpc_client, cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
) -> ApiResult<Json<serde_json::Value>> {
    let cache_key = "blockdag_info";

    if let Some(cached) = cache.get(cache_key).await {
        return Ok(Json(cached));
    }

    let info = rpc_client.get_blockdag_info().await?;
    let json = serde_json::to_value(&info)?;
    
    cache.set(cache_key.to_string(), json.clone()).await;

    Ok(Json(json))
}

// ============================================
// Coin Supply
// ============================================

pub async fn get_coin_supply(
    State((rpc_client, cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
) -> ApiResult<Json<serde_json::Value>> {
    let cache_key = "coin_supply";

    if let Some(cached) = cache.get(cache_key).await {
        return Ok(Json(cached));
    }

    let supply = rpc_client.get_coin_supply().await?;
    let json = serde_json::to_value(&supply)?;
    
    cache.set(cache_key.to_string(), json.clone()).await;

    Ok(Json(json))
}

// ============================================
// Hashrate
// ============================================

pub async fn get_hashrate(
    State((rpc_client, cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
) -> ApiResult<Json<serde_json::Value>> {
    let cache_key = "hashrate";

    if let Some(cached) = cache.get(cache_key).await {
        return Ok(Json(cached));
    }

    let hashrate = rpc_client.get_hashrate().await?;
    let json = serde_json::json!({
        "hashrate": hashrate,
        "unit": "H/s"
    });
    
    cache.set(cache_key.to_string(), json.clone()).await;

    Ok(Json(json))
}

pub async fn get_hashrate_max(
    State((_rpc_client, _cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "hashrate": 9876543210u64,
        "unit": "H/s"
    })))
}

// ============================================
// Fee Estimate
// ============================================

pub async fn get_fee_estimate(
    State((rpc_client, cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
) -> ApiResult<Json<serde_json::Value>> {
    let cache_key = "fee_estimate";

    if let Some(cached) = cache.get(cache_key).await {
        return Ok(Json(cached));
    }

    let estimate = rpc_client.get_fee_estimate().await?;
    let json = serde_json::to_value(&estimate)?;
    
    cache.set(cache_key.to_string(), json.clone()).await;

    Ok(Json(json))
}

// ============================================
// Halving Info
// ============================================

pub async fn get_halving(
    State((_rpc_client, _cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "next_halving_date": "2025-12-31",
        "blocks_until_halving": 1000000
    })))
}

// ============================================
// Market Data
// ============================================

pub async fn get_market_data(
    State((_rpc_client, cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
) -> ApiResult<Json<serde_json::Value>> {
    let cache_key = "market_data";

    if let Some(cached) = cache.get(cache_key).await {
        return Ok(Json(cached));
    }

    let market_data = serde_json::json!({
        "current_price": { "usd": 0.50 },
        "price_change_percentage_24h": 5.2,
        "market_cap": { "usd": 14350000000.0 },
        "total_volume": { "usd": 500000000.0 }
    });

    cache.set(cache_key.to_string(), market_data.clone()).await;

    Ok(Json(market_data))
}
