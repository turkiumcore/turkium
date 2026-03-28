use crate::database::Database;
use crate::miner::MinerManager;
use crate::share::ShareProcessor;
use crate::stratum::message::{StratumMessage, StratumRequest};
use crate::stratum::protocol::StratumCodec;
use anyhow::Result;
use log::{error, info, warn};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};

pub async fn run(
    listen_addr: &str,
    db: Arc<Database>,
    miner_manager: Arc<MinerManager>,
    share_processor: Arc<ShareProcessor>,
) -> Result<()> {
    let listener = TcpListener::bind(listen_addr).await?;
    info!("Stratum server listening on {}", listen_addr);

    loop {
        let (socket, peer_addr) = listener.accept().await?;
        info!("New connection from {}", peer_addr);

        let db = db.clone();
        let miner_mgr = miner_manager.clone();
        let share_proc = share_processor.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, db, miner_mgr, share_proc).await {
                warn!("Client {} error: {:?}", peer_addr, e);
            }
            info!("Client {} disconnected", peer_addr);
        });
    }
}

async fn handle_client(
    mut socket: TcpStream,
    db: Arc<Database>,
    miner_manager: Arc<MinerManager>,
    share_processor: Arc<ShareProcessor>,
) -> Result<()> {
    let mut miner_id: Option<i64> = None;
    let mut username: Option<String> = None;

    loop {
        match StratumCodec::read_message(&mut socket).await {
            Ok(req) => {
                match StratumMessage::parse(&req) {
                    Ok(msg) => {
                        match msg {
                            StratumMessage::Subscribe { id, user_agent } => {
                                info!("Subscribe request from {}", user_agent);
                                let response = msg.to_response();
                                StratumCodec::write_message(&mut socket, &response).await?;
                            }
                            StratumMessage::Authorize { id, username: user, password } => {
                                info!("Authorize request for {}", user);
                                
                                // Register or get miner
                                let mid = crate::database::get_or_create_miner(
                                    db.pool(),
                                    &user,
                                    &user,
                                )
                                .await?;
                                
                                miner_id = Some(mid);
                                username = Some(user.clone());
                                
                                // Add to manager
                                miner_manager.add_miner(mid, user.clone(), &mut socket).await?;
                                
                                let response = msg.to_response();
                                StratumCodec::write_message(&mut socket, &response).await?;
                                
                                // Send initial difficulty
                                let diff_msg = StratumMessage::SetDifficulty { difficulty: 1.0 };
                                if let Some(json) = diff_msg.to_notify_json() {
                                    StratumCodec::write_raw(&mut socket, &json).await?;
                                }
                                
                                // Send initial block template
                                if let Ok(Some((job_id, prev_hash, cb1, cb2, merkle, version, nbits, ntime, _))) =
                                    crate::database::get_latest_block_template(db.pool()).await
                                {
                                    let notify_msg = StratumMessage::Notify {
                                        job_id,
                                        prev_hash,
                                        coinbase1: cb1,
                                        coinbase2: cb2,
                                        merkle_branch: merkle.split(',').map(|s| s.to_string()).collect(),
                                        version,
                                        nbits,
                                        ntime,
                                        clean_jobs: true,
                                    };
                                    if let Some(json) = notify_msg.to_notify_json() {
                                        StratumCodec::write_raw(&mut socket, &json).await?;
                                    }
                                }
                            }
                            StratumMessage::Submit { id, username: user, job_id, nonce, result } => {
                                if let Some(mid) = miner_id {
                                    // Process share
                                    match share_processor.process_share(
                                        mid,
                                        &job_id,
                                        &nonce,
                                        &result,
                                    )
                                    .await
                                    {
                                        Ok((is_block, difficulty)) => {
                                            if is_block {
                                                info!("Block found by miner {}", user);
                                            }
                                            let response = msg.to_response();
                                            StratumCodec::write_message(&mut socket, &response).await?;
                                        }
                                        Err(e) => {
                                            error!("Share processing error: {}", e);
                                            let response = StratumMessage::Submit {
                                                id,
                                                username: user,
                                                job_id,
                                                nonce,
                                                result,
                                            };
                                            let resp = response.to_response();
                                            StratumCodec::write_message(&mut socket, &resp).await?;
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        error!("Parse error: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Read error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
