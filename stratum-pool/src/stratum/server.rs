use crate::database::Database;
use crate::miner::MinerManager;
use crate::share::ShareProcessor;
use crate::stratum::protocol::StratumCodec;
use crate::stratum::message::StratumMessage;
use anyhow::Result;
use log::{error, info, warn};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

pub async fn run(
    listen_addr: &str,
    db: Arc<Database>,
    miner_manager: Arc<MinerManager>,
    share_processor: Arc<ShareProcessor>,
) -> Result<()> {
    let listener = TcpListener::bind(listen_addr).await?;
    info!("Stratum server listening on {}", listen_addr);

    // Create broadcast channel for block template updates
    // This allows all connected miners to receive new work when block template changes
    let (tx, _rx) = broadcast::channel::<String>(100);
    let tx = Arc::new(tx);

    loop {
        let (socket, peer_addr) = listener.accept().await?;
        info!("New connection from {}", peer_addr);

        let db = db.clone();
        let miner_mgr = miner_manager.clone();
        let share_proc = share_processor.clone();
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, db, miner_mgr, share_proc, tx_clone).await {
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
    template_broadcast: Arc<broadcast::Sender<String>>,
) -> Result<()> {
    let mut miner_id: Option<i64> = None;
    let mut miner_rx = template_broadcast.subscribe();

    loop {
        tokio::select! {
            // Handle incoming messages from miner
            result = StratumCodec::read_message(&mut socket) => {
                match result {
                    Ok(req) => {
                        // Parse the stratum message
                        match StratumMessage::parse(&req) {
                            Ok(msg) => {
                                match msg {
                                    StratumMessage::Subscribe { id, user_agent } => {
                                        info!("Miner subscribed: {} ({})", id, user_agent);
                                        
                                        let response = serde_json::json!({
                                            "id": id,
                                            "result": [
                                                [["mining.set_difficulty", "1"], ["mining.notify", "1"]],
                                                "00000000",
                                                2
                                            ],
                                            "error": null
                                        });
                                        
                                        let json_str = response.to_string();
                                        socket.write_all(json_str.as_bytes()).await?;
                                        socket.write_all(b"\n").await?;
                                        socket.flush().await?;
                                    }
                                    StratumMessage::Authorize { id, username } => {
                                        info!("Miner authorized: {}", username);
                                        
                                        // Get or create miner in database
                                        match crate::database::get_or_create_miner(db.pool(), &username, &username).await {
                                            Ok(mid) => {
                                                miner_id = Some(mid);
                                                
                                                // Add to miner manager
                                                let _ = miner_manager.add_miner(mid, username.clone()).await;
                                                
                                                let response = serde_json::json!({
                                                    "id": id,
                                                    "result": true,
                                                    "error": null
                                                });
                                                
                                                let json_str = response.to_string();
                                                socket.write_all(json_str.as_bytes()).await?;
                                                socket.write_all(b"\n").await?;
                                                socket.flush().await?;
                                            }
                                            Err(e) => {
                                                error!("Failed to authorize miner: {}", e);
                                                let response = serde_json::json!({
                                                    "id": id,
                                                    "result": null,
                                                    "error": "Authorization failed"
                                                });
                                                
                                                let json_str = response.to_string();
                                                socket.write_all(json_str.as_bytes()).await?;
                                                socket.write_all(b"\n").await?;
                                                socket.flush().await?;
                                            }
                                        }
                                    }
                                    StratumMessage::Submit { username, job_id, nonce, result } => {
                                        info!("Share submitted by {}: job_id={}, nonce={}", username, job_id, nonce);
                                        
                                        if let Some(mid) = miner_id {
                                            // Process the share
                                            match share_processor.process_share(mid, &job_id, &nonce, &result).await {
                                                Ok((is_block, difficulty)) => {
                                                    info!("Share accepted: difficulty={}, is_block={}", difficulty, is_block);
                                                    
                                                    let response = serde_json::json!({
                                                        "id": req.id,
                                                        "result": true,
                                                        "error": null
                                                    });
                                                    
                                                    let json_str = response.to_string();
                                                    socket.write_all(json_str.as_bytes()).await?;
                                                    socket.write_all(b"\n").await?;
                                                    socket.flush().await?;
                                                }
                                                Err(e) => {
                                                    warn!("Failed to process share: {}", e);
                                                    let response = serde_json::json!({
                                                        "id": req.id,
                                                        "result": null,
                                                        "error": "Share rejected"
                                                    });
                                                    
                                                    let json_str = response.to_string();
                                                    socket.write_all(json_str.as_bytes()).await?;
                                                    socket.write_all(b"\n").await?;
                                                    socket.flush().await?;
                                                }
                                            }
                                        } else {
                                            warn!("Share submitted before authorization");
                                            let response = serde_json::json!({
                                                "id": req.id,
                                                "result": null,
                                                "error": "Not authorized"
                                            });
                                            
                                            let json_str = response.to_string();
                                            socket.write_all(json_str.as_bytes()).await?;
                                            socket.write_all(b"\n").await?;
                                            socket.flush().await?;
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to parse message: {}", e);
                                let response = serde_json::json!({
                                    "id": 0,
                                    "result": null,
                                    "error": "Invalid message"
                                });
                                
                                let json_str = response.to_string();
                                socket.write_all(json_str.as_bytes()).await?;
                                socket.write_all(b"\n").await?;
                                socket.flush().await?;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Read error: {}", e);
                        break;
                    }
                }
            }
            
            // Handle block template updates from broadcast channel
            Ok(template_update) = miner_rx.recv() => {
                // Production logic: Send mining.notify to all connected miners
                // This tells miners to update their work with new block template
                if let Err(e) = socket.write_all(template_update.as_bytes()).await {
                    error!("Failed to send template update to miner: {}", e);
                    break;
                }
                if let Err(e) = socket.write_all(b"\n").await {
                    error!("Failed to send newline to miner: {}", e);
                    break;
                }
                if let Err(e) = socket.flush().await {
                    error!("Failed to flush socket: {}", e);
                    break;
                }
            }
        }
    }

    Ok(())
}
