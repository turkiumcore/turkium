mod config;
mod error;
mod handlers;
mod models;
mod rpc_client;
mod cache;
mod middleware;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

use crate::config::Config;
use crate::rpc_client::TurkiumRpcClient;
use crate::cache::ResponseCache;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("turkium_api_server=debug".parse().unwrap()),
        )
        .init();

    // Load configuration
    dotenv::dotenv().ok();
    let config = Config::from_env()?;

    tracing::info!("🚀 Turkium API Server v1.0.0");
    tracing::info!("📍 Server: {}:{}", config.server_host, config.server_port);
    tracing::info!("🔗 Node: {}:{}", config.turkium_node_host, config.turkium_node_port);
    tracing::info!("🌍 Environment: {}", config.environment);

    // Initialize RPC client
    let rpc_client = Arc::new(
        TurkiumRpcClient::new(
            &config.turkium_node_host,
            config.turkium_node_port,
            config.turkium_node_timeout_secs,
        )
        .await?,
    );

    // Initialize cache
    let cache = Arc::new(ResponseCache::new(
        config.cache_ttl_secs,
        config.cache_max_capacity,
    ));

    // Test connection to node
    match rpc_client.ping().await {
        Ok(_) => tracing::info!("✅ Connected to turkium node"),
        Err(e) => {
            tracing::warn!("⚠️  Failed to connect to turkium node: {}", e);
            tracing::warn!("⚠️  Server will start but endpoints may fail");
        }
    }

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health::health_check))
        
        // Info endpoints
        .route("/info/blockdag", get(handlers::info::get_blockdag_info))
        .route("/info/coinsupply", get(handlers::info::get_coin_supply))
        .route("/info/hashrate", get(handlers::info::get_hashrate))
        .route("/info/hashrate/max", get(handlers::info::get_hashrate_max))
        .route("/info/fee-estimate", get(handlers::info::get_fee_estimate))
        .route("/info/halving", get(handlers::info::get_halving))
        .route("/info/market-data", get(handlers::info::get_market_data))
        
        // Block endpoints
        .route("/blocks/:hash", get(handlers::blocks::get_block))
        
        // Transaction endpoints
        .route("/transactions/:hash", get(handlers::transactions::get_transaction))
        .route("/transactions/count", get(handlers::transactions::get_transaction_count))
        .route("/transactions/search", post(handlers::transactions::search_transactions))
        
        // Address endpoints
        .route("/addresses/:addr/balance", get(handlers::addresses::get_address_balance))
        .route("/addresses/:addr/transactions-count", get(handlers::addresses::get_address_tx_count))
        .route("/addresses/:addr/utxos", get(handlers::addresses::get_address_utxos))
        .route("/addresses/:addr/full-transactions", get(handlers::addresses::get_address_transactions))
        .route("/addresses/:addr/name", get(handlers::addresses::get_address_name))
        
        // Add middleware
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(128 * 1024 * 1024)) // 128MB
        .layer(axum::middleware::from_fn(crate::middleware::logging_middleware))
        .with_state((rpc_client, cache));

    // Start server
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.server_host, config.server_port
    ))
    .await?;

    tracing::info!("✅ Server listening on http://{}:{}", config.server_host, config.server_port);
    tracing::info!("📚 API Documentation: http://{}:{}/health", config.server_host, config.server_port);

    axum::serve(listener, app).await?;

    Ok(())
}
