use anyhow::Result;
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;
use turkium_grpc_client::GrpcClient;
use turkium_rpc_core::api::rpc::RpcApi;
use turkium_rpc_core::notify::mode::NotificationMode;
use turkium_rpc_core::{RpcAddress, RpcExtraData};

pub struct BlockTemplate {
    pub prev_hash: String,
    pub coinbase1: String,
    pub coinbase2: String,
    pub merkle_branch: Vec<String>,
    pub version: String,
    pub nbits: String,
    pub ntime: String,
    pub height: i64,
    pub is_synced: bool,
}

pub struct BlockchainClient {
    client: Option<GrpcClient>,
}

impl BlockchainClient {
    pub async fn new(grpc_address: &str) -> Result<Self> {
        info!("Blockchain client initialized for: {}", grpc_address);
        
        // Connect to the gRPC server using GrpcClient
        let client = match GrpcClient::connect_with_args(
            NotificationMode::Direct,
            grpc_address.to_string(),
            None,
            false,
            None,
            false,
            Some(500_000),
            Default::default(),
        )
        .await
        {
            Ok(c) => Some(c),
            Err(e) => {
                log::warn!("Failed to connect to blockchain gRPC at {}: {}", grpc_address, e);
                None
            }
        };
        
        Ok(Self {
            client,
        })
    }

    pub async fn get_block_template(&mut self) -> Result<BlockTemplate> {
        let client = self.client.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Blockchain client not connected"))?;
        
        // Create mining address
        let pay_address = RpcAddress::try_from("Turkium:qzq34kp6kxqrput2n60sf9qdaealtpwlrd4nhnets479kd3nwd3t6xgv9xd45")?;
        let extra_data = RpcExtraData::from_iter(vec![]);
        
        // Call the RPC to get block template
        let response = client.get_block_template(pay_address, extra_data).await?;
        
        let block = &response.block;
        let header = &block.header;
        
        // Production logic: Extract is_synced from RPC response
        // This indicates whether the blockchain node considers itself synced
        // In isolated mode (no peers), is_synced will be true if the node has a valid block template
        let is_synced = response.is_synced;
        
        // For mining, use hash_merkle_root as the previous hash reference
        Ok(BlockTemplate {
            prev_hash: header.hash_merkle_root.to_string(),
            coinbase1: String::new(),
            coinbase2: String::new(),
            merkle_branch: vec![],
            version: header.version.to_string(),
            nbits: format!("{:08x}", header.bits),
            ntime: (header.timestamp / 1000).to_string(), // Convert milliseconds to seconds
            height: header.daa_score as i64,
            is_synced,
        })
    }
}

pub async fn template_updater(
    blockchain: Arc<Mutex<BlockchainClient>>,
    db: Arc<crate::database::Database>,
    template_tx: Arc<tokio::sync::broadcast::Sender<String>>,
) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));

    loop {
        interval.tick().await;

        // Lock the blockchain client and get block template
        let mut client = blockchain.lock().await;
        match client.get_block_template().await {
            Ok(template) => {
                // Production logic: Only save block template if blockchain is synced
                // This ensures miners only work on valid block templates
                if !template.is_synced {
                    log::warn!("Blockchain not synced, skipping block template update");
                    continue;
                }

                let job_id = format!("{:x}", chrono::Utc::now().timestamp());
                let merkle_str = template.merkle_branch.join(",");

                if let Err(e) = crate::database::save_block_template(
                    db.pool(),
                    &job_id,
                    &template.prev_hash,
                    &template.coinbase1,
                    &template.coinbase2,
                    &merkle_str,
                    &template.version,
                    &template.nbits,
                    &template.ntime,
                    template.height,
                )
                .await
                {
                    log::error!("Failed to save block template: {}", e);
                    continue;
                }

                // Production logic: Broadcast new block template to all connected miners
                // This tells miners to update their work with the new template
                let notify_message = serde_json::json!({
                    "id": null,
                    "method": "mining.notify",
                    "params": [
                        job_id,
                        template.prev_hash,
                        template.coinbase1,
                        template.coinbase2,
                        template.merkle_branch,
                        template.version,
                        template.nbits,
                        template.ntime,
                        true
                    ]
                });

                let notify_str = notify_message.to_string();
                if let Err(e) = template_tx.send(notify_str) {
                    log::debug!("No miners connected to receive template update: {}", e);
                }
            }
            Err(e) => {
                log::error!("Failed to get block template: {}", e);
            }
        }
    }
}
