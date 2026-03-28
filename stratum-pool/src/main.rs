mod config;
mod database;
mod stratum;
mod blockchain;
mod miner;
mod share;
mod reward;

use anyhow::Result;
use clap::Parser;
use log::info;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "Turkium Stratum Pool")]
#[command(about = "High-performance mining pool for Turkium", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    #[arg(short, long)]
    listen: Option<String>,

    #[arg(short, long)]
    blockchain: Option<String>,

    #[arg(short, long)]
    database: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Args::parse();

    // Load configuration
    let mut cfg = config::Config::from_file(&args.config).await?;
    
    // Override with CLI arguments
    if let Some(listen) = args.listen {
        cfg.pool.listen_address = listen;
    }
    if let Some(blockchain) = args.blockchain {
        cfg.blockchain.grpc_address = blockchain;
    }
    if let Some(database) = args.database {
        cfg.database.path = database;
    }

    info!("Turkium Stratum Pool v0.1.0");
    info!("Listening on: {}", cfg.pool.listen_address);
    info!("Blockchain: {}", cfg.blockchain.grpc_address);
    info!("Database: {}", cfg.database.path);

    // Initialize database (auto-create and migrate)
    let db = database::Database::new(&cfg.database.path).await?;
    db.migrate().await?;
    let db = Arc::new(db);

    info!("Database initialized");

    // Initialize blockchain client
    let blockchain_client = blockchain::BlockchainClient::new(&cfg.blockchain.grpc_address).await?;
    let blockchain_client = Arc::new(blockchain_client);

    info!("Connected to blockchain");

    // Initialize miner manager
    let miner_manager = Arc::new(miner::MinerManager::new(db.clone()));

    // Initialize share processor
    let share_processor = Arc::new(share::ShareProcessor::new(
        db.clone(),
        blockchain_client.clone(),
        cfg.pool.clone(),
    ));

    // Initialize reward distributor
    let reward_distributor = Arc::new(reward::RewardDistributor::new(
        db.clone(),
        cfg.pool.clone(),
    ));

    // Start block template updater
    let template_updater = {
        let blockchain = blockchain_client.clone();
        let db = db.clone();
        tokio::spawn(async move {
            blockchain::template_updater(blockchain, db).await;
        })
    };

    // Start reward distributor
    let reward_task = {
        let distributor = reward_distributor.clone();
        tokio::spawn(async move {
            distributor.run().await;
        })
    };

    // Start Stratum server
    let stratum_server = {
        let listen_addr = cfg.pool.listen_address.clone();
        let db = db.clone();
        let miner_mgr = miner_manager.clone();
        let share_proc = share_processor.clone();
        tokio::spawn(async move {
            if let Err(e) = stratum::server::run(
                &listen_addr,
                db,
                miner_mgr,
                share_proc,
            )
            .await
            {
                log::error!("Stratum server error: {}", e);
            }
        })
    };

    info!("Pool started successfully");

    // Wait for all tasks
    tokio::select! {
        _ = template_updater => info!("Template updater stopped"),
        _ = reward_task => info!("Reward distributor stopped"),
        _ = stratum_server => info!("Stratum server stopped"),
    }

    Ok(())
}
