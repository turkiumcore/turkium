use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::cache::ResponseCache;
use crate::error::ApiResult;
use crate::models::*;
use crate::rpc_client::TurkiumRpcClient;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_limit")]
    limit: u32,
    #[serde(default)]
    offset: u32,
}

fn default_limit() -> u32 {
    20
}

pub async fn get_address_balance(
    State((rpc_client, cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
    Path(addr): Path<String>,
) -> ApiResult<Json<AddressBalance>> {
    let cache_key = format!("addr_balance:{}", addr);

    if let Some(cached) = cache.get(&cache_key).await {
        if let Ok(balance) = serde_json::from_value::<AddressBalance>(cached) {
            return Ok(Json(balance));
        }
    }

    let balance_amount = rpc_client.get_address_balance(&addr).await?;
    let balance = AddressBalance {
        address: addr.clone(),
        balance: balance_amount,
    };
    
    cache
        .set(
            cache_key,
            serde_json::to_value(&balance).unwrap_or_default(),
        )
        .await;

    Ok(Json(balance))
}

pub async fn get_address_tx_count(
    State((_rpc_client, cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
    Path(addr): Path<String>,
) -> ApiResult<Json<AddressTxCount>> {
    let cache_key = format!("addr_tx_count:{}", addr);

    if let Some(cached) = cache.get(&cache_key).await {
        if let Ok(count) = serde_json::from_value::<AddressTxCount>(cached) {
            return Ok(Json(count));
        }
    }

    // Note: Turkium gRPC doesn't provide direct transaction count per address
    // This would require scanning the blockchain or maintaining an index
    let count = AddressTxCount {
        address: addr.clone(),
        transaction_count: 0,
    };

    cache
        .set(
            cache_key,
            serde_json::to_value(&count).unwrap_or_default(),
        )
        .await;

    Ok(Json(count))
}

pub async fn get_address_utxos(
    State((rpc_client, cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
    Path(addr): Path<String>,
) -> ApiResult<Json<Vec<UTXO>>> {
    let cache_key = format!("addr_utxos:{}", addr);

    if let Some(cached) = cache.get(&cache_key).await {
        if let Ok(utxos) = serde_json::from_value::<Vec<UTXO>>(cached) {
            return Ok(Json(utxos));
        }
    }

    let utxos_raw = rpc_client.get_address_utxos(&addr).await?;
    let utxos: Vec<UTXO> = utxos_raw
        .into_iter()
        .map(|(txid, index, amount)| UTXO {
            transaction_id: txid,
            output_index: index,
            amount,
            script_public_key: String::new(),
            block_daa_score: 0,
        })
        .collect();

    cache
        .set(
            cache_key,
            serde_json::to_value(&utxos).unwrap_or_default(),
        )
        .await;

    Ok(Json(utxos))
}

pub async fn get_address_transactions(
    State((_rpc_client, _cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
    Path(addr): Path<String>,
    Query(params): Query<PaginationParams>,
) -> ApiResult<Json<AddressTransactions>> {
    // Note: Turkium gRPC doesn't provide direct transaction list per address
    // This would require maintaining an address index or scanning the blockchain
    let transactions = Vec::new();

    Ok(Json(AddressTransactions {
        address: addr,
        transactions,
        limit: params.limit,
        offset: params.offset,
    }))
}

pub async fn get_address_name(
    State((_rpc_client, _cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
    Path(addr): Path<String>,
) -> ApiResult<Json<AddressName>> {
    // Address naming is not supported by Turkium gRPC
    Ok(Json(AddressName {
        address: addr,
        name: None,
    }))
}
