#![allow(non_snake_case)]
#![cfg_attr(all(test, feature = "bench"), feature(test))]

use chrono::Local;
use clap::Parser;
use log::{debug, info, warn};
use std::error::Error as StdError;
use std::{
    io::Write,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use crate::{
    cli::Opt, miner::MinerManager,
    target::Uint256,
};

mod cli;
mod miner;
mod stratum_client;
mod swap_rust;
mod target;
mod turkiumd_messages;

pub mod proto {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("protowire");
}

pub type Error = Box<dyn StdError + Send + Sync + 'static>;

type Hash = Uint256;

#[derive(Debug, Clone)]
pub struct ShutdownHandler(Arc<AtomicBool>);

pub struct ShutdownOnDrop(ShutdownHandler);

impl ShutdownHandler {
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }

    #[inline(always)]
    pub fn arm(&self) -> ShutdownOnDrop {
        ShutdownOnDrop(self.clone())
    }
}

impl Drop for ShutdownOnDrop {
    fn drop(&mut self) {
        self.0 .0.store(true, Ordering::Release);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut opt: Opt = Opt::parse();
    opt.process()?;

    let mut builder = env_logger::builder();
    builder.filter_level(opt.log_level()).parse_default_env();
    if opt.altlogs {
        builder.format(|buf, record| {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f%:z");
            writeln!(buf, "{} [{:>5}] {}", timestamp, record.level(), record.args())
        });
    }
    builder.init();

    let throttle = opt.throttle.map(Duration::from_millis);
    let shutdown = ShutdownHandler(Arc::new(AtomicBool::new(false)));
    let _shutdown_when_dropped = shutdown.arm();

    // Only pool mode is supported
    info!("Starting Stratum pool miner");
    run_pool_mode(opt, throttle, shutdown).await
}

async fn run_pool_mode(
    opt: Opt,
    throttle: Option<Duration>,
    shutdown: ShutdownHandler,
) -> Result<(), Error> {
    let pool_address = opt.pool_address.clone().unwrap_or_else(|| "127.0.0.1:3333".to_string());
    
    info!("Connecting to Stratum pool at {}", pool_address);

    let (mut stratum_client, mut template_rx) = stratum_client::StratumClient::connect(
        &pool_address,
        opt.mining_address.clone(),
    )
    .await?;

    info!("Connected to Stratum pool");

    // Spawn listener task
    let listener_shutdown = shutdown.clone();
    let listener_task = tokio::spawn(async move {
        if let Err(e) = stratum_client.listen_for_notifications().await {
            warn!("Pool listener error: {}", e);
        }
    });

    // Mine with templates from pool
    let mut miner_manager = MinerManager::new(
        // For pool mode, we don't use the gRPC channel
        // Instead we'll handle submissions differently
        tokio::sync::mpsc::channel(1).0,
        opt.num_threads,
        throttle,
        shutdown.clone(),
    );

    while let Some(template) = template_rx.recv().await {
        if shutdown.is_shutdown() {
            break;
        }

        debug!("Received template from pool: job_id={}", template.job_id);
        // Process template for mining
        // This would integrate with the miner manager
    }

    listener_task.abort();
    Ok(())
}
