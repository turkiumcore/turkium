mod crawler;
mod dns_server;
mod peer_manager;
mod config;

use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(name = "turkium DNS Seeder")]
#[command(about = "DNS seeder for turkium blockchain peer discovery", long_about = None)]
struct Args {
    /// DNS seeder hostname (e.g., mainnet-seeder-1.turkium.org)
    #[arg(long, default_value = "localhost")]
    hostname: String,

    /// turkium P2P port to crawl
    #[arg(long, default_value = "5206")]
    peer_port: u16,

    /// DNS server listen port
    #[arg(long, default_value = "53")]
    dns_port: u16,

    /// DNS server listen address
    #[arg(long, default_value = "0.0.0.0")]
    dns_listen: String,

    /// Initial peer to bootstrap from (e.g., 188.132.197.20:5206)
    #[arg(long)]
    bootstrap_peer: Option<String>,

    /// Network type (mainnet, testnet, devnet)
    #[arg(long, default_value = "mainnet")]
    network: String,

    /// Minimum number of peers to maintain
    #[arg(long, default_value = "50")]
    min_peers: usize,

    /// Maximum number of peers to track
    #[arg(long, default_value = "500")]
    max_peers: usize,

    /// Crawl interval in seconds
    #[arg(long, default_value = "300")]
    crawl_interval: u64,

    /// Peer timeout in seconds
    #[arg(long, default_value = "600")]
    peer_timeout: u64,

    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&args.log_level)),
        )
        .init();

    tracing::info!("Starting turkium DNS Seeder");
    tracing::info!("Hostname: {}", args.hostname);
    tracing::info!("Network: {}", args.network);
    tracing::info!("DNS Listen: {}:{}", args.dns_listen, args.dns_port);
    tracing::info!("Peer Port: {}", args.peer_port);

    // Create configuration
    let config = config::Config {
        hostname: args.hostname.clone(),
        peer_port: args.peer_port,
        dns_port: args.dns_port,
        dns_listen: args.dns_listen.clone(),
        bootstrap_peer: args.bootstrap_peer.clone(),
        network: args.network.clone(),
        min_peers: args.min_peers,
        max_peers: args.max_peers,
        crawl_interval: args.crawl_interval,
        peer_timeout: args.peer_timeout,
    };

    // Create peer manager
    let peer_manager = Arc::new(peer_manager::PeerManager::new(config.clone()));

    // Start crawler
    let crawler = crawler::Crawler::new(peer_manager.clone(), config.clone());
    let crawler_handle = tokio::spawn(async move {
        if let Err(e) = crawler.run().await {
            tracing::error!("Crawler error: {}", e);
        }
    });

    // Start DNS server
    let dns_server = dns_server::DnsServer::new(peer_manager.clone(), config.clone());
    let dns_handle = tokio::spawn(async move {
        if let Err(e) = dns_server.run().await {
            tracing::error!("DNS server error: {}", e);
        }
    });

    // Wait for both tasks
    tokio::select! {
        _ = crawler_handle => {
            tracing::error!("Crawler task ended unexpectedly");
        }
        _ = dns_handle => {
            tracing::error!("DNS server task ended unexpectedly");
        }
    }

    Ok(())
}
