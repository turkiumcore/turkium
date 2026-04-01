use axum::{extract::State, Json};
use chrono::Utc;
use std::sync::Arc;

use crate::cache::ResponseCache;
use crate::error::ApiResult;
use crate::models::HealthResponse;
use crate::rpc_client::TurkiumRpcClient;

pub async fn health_check(
    State((rpc_client, _cache)): State<(Arc<TurkiumRpcClient>, Arc<ResponseCache>)>,
) -> ApiResult<Json<HealthResponse>> {
    let node_connected = rpc_client.ping().await.is_ok();

    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        timestamp: Utc::now(),
        node_connected,
    }))
}
