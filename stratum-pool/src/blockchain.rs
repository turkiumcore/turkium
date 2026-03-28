use anyhow::Result;
use log::info;
use std::sync::Arc;
use tonic::transport::Channel;

pub mod proto {
    tonic::include_proto!("protowire");
}

use proto::rpc_client::RpcClient;
use proto::GetBlockTemplateRequestMessage;

pub struct BlockchainClient {
    client: RpcClient<Channel>,
}

impl BlockchainClient {
    pub async fn new(grpc_address: &str) -> Result<Self> {
        let channel = Channel::from_shared(grpc_address.to_string())?
            .connect()
            .await?;

        let client = RpcClient::new(channel);
        Ok(Self { client })
    }

    pub async fn get_block_template(&mut self) -> Result<BlockTemplate> {
        let request = GetBlockTemplateRequestMessage {};
        let response = self.client.get_block_template(request).await?;
        let msg = response.into_inner();

        Ok(BlockTemplate {
            prev_hash: msg.previous_block_hash,
            coinbase1: msg.coinbase_1,
            coinbase2: msg.coinbase_2,
            merkle_branch: msg.merkle_branch,
            version: msg.version,
            nbits: msg.bits,
            ntime: msg.current_time,
            height: msg.block_number,
        })
    }
}

pub struct BlockTemplate {
    pub prev_hash: String,
    pub coinbase1: String,
    pub coinbase2: String,
    pub merkle_branch: Vec<String>,
    pub version: String,
    pub nbits: String,
    pub ntime: String,
    pub height: i64,
}

pub async fn template_updater(
    mut blockchain: Arc<BlockchainClient>,
    db: Arc<crate::database::Database>,
) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));

    loop {
        interval.tick().await;

        match blockchain.get_block_template().await {
            Ok(template) => {
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
                }
            }
            Err(e) => {
                log::error!("Failed to get block template: {}", e);
            }
        }
    }
}
