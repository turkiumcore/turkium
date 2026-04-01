use anyhow::Result;
use std::sync::Arc;
use turkium_grpc_client::GrpcClient;
use turkium_rpc_core::notify::mode::NotificationMode;
use turkium_rpc_core::{
    GetBlockDagInfoResponse, GetServerInfoResponse, GetCoinSupplyResponse,
    RpcFeeEstimate, RpcAddress,
    api::rpc::RpcApi,
};

/// Turkium RPC Client wrapper
pub struct TurkiumRpcClient {
    client: Arc<GrpcClient>,
}

impl TurkiumRpcClient {
    /// Create a new RPC client and connect to the node
    pub async fn new(
        host: &str,
        port: u16,
        _timeout_secs: u64,
    ) -> Result<Self> {
        let url = format!("grpc://{}:{}", host, port);
        
        tracing::info!("Connecting to turkium node at: {}", url);
        
        let client = GrpcClient::connect_with_args(
            NotificationMode::Direct,
            url,
            None,
            false,
            None,
            false,
            Some(500_000),
            Default::default(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to gRPC server: {}", e))?;

        Ok(TurkiumRpcClient {
            client: Arc::new(client),
        })
    }

    /// Test connection to the node
    pub async fn ping(&self) -> Result<()> {
        tracing::debug!("RPC Call: Ping");
        
        self.client
            .get_server_info()
            .await
            .map_err(|e| {
                tracing::error!("RPC Connection Error: {}", e);
                anyhow::anyhow!("Node Connection Error: Failed to connect to node: {}", e)
            })?;

        Ok(())
    }

    /// Get blockdag info
    pub async fn get_blockdag_info(&self) -> Result<GetBlockDagInfoResponse> {
        tracing::debug!("RPC Call: GetBlockDagInfo");
        
        self.client
            .get_block_dag_info()
            .await
            .map_err(|e| {
                tracing::error!("RPC Error: {}", e);
                anyhow::anyhow!("Failed to get blockdag info: {}", e)
            })
    }

    /// Get server info
    #[allow(dead_code)]
    pub async fn get_server_info(&self) -> Result<GetServerInfoResponse> {
        tracing::debug!("RPC Call: GetServerInfo");
        
        self.client
            .get_server_info()
            .await
            .map_err(|e| {
                tracing::error!("RPC Error: {}", e);
                anyhow::anyhow!("Failed to get server info: {}", e)
            })
    }

    /// Get coin supply
    pub async fn get_coin_supply(&self) -> Result<GetCoinSupplyResponse> {
        tracing::debug!("RPC Call: GetCoinSupply");
        
        self.client
            .get_coin_supply()
            .await
            .map_err(|e| {
                tracing::error!("RPC Error: {}", e);
                anyhow::anyhow!("Failed to get coin supply: {}", e)
            })
    }

    /// Get hashrate (network hashes per second)
    pub async fn get_hashrate(&self) -> Result<u64> {
        tracing::debug!("RPC Call: GetHashRate");
        
        self.client
            .estimate_network_hashes_per_second(100, None)
            .await
            .map_err(|e| {
                tracing::error!("RPC Error: {}", e);
                anyhow::anyhow!("Failed to get hashrate: {}", e)
            })
    }

    /// Get fee estimate
    pub async fn get_fee_estimate(&self) -> Result<RpcFeeEstimate> {
        tracing::debug!("RPC Call: GetFeeEstimate");
        
        self.client
            .get_fee_estimate()
            .await
            .map_err(|e| {
                tracing::error!("RPC Error: {}", e);
                anyhow::anyhow!("Failed to get fee estimate: {}", e)
            })
    }

    /// Get balance for a single address
    pub async fn get_address_balance(&self, address: &str) -> Result<u64> {
        tracing::debug!("RPC Call: GetBalanceByAddress({})", address);
        
        let rpc_address = RpcAddress::try_from(address.to_string())
            .map_err(|e| anyhow::anyhow!("Invalid address format: {}", e))?;
        
        self.client
            .get_balance_by_address(rpc_address)
            .await
            .map_err(|e| {
                tracing::error!("RPC Error: {}", e);
                anyhow::anyhow!("Failed to get address balance: {}", e)
            })
    }

    /// Get UTXOs for a single address
    pub async fn get_address_utxos(&self, address: &str) -> Result<Vec<(String, u32, u64)>> {
        tracing::debug!("RPC Call: GetUtxosByAddresses({})", address);
        
        let rpc_address = RpcAddress::try_from(address.to_string())
            .map_err(|e| anyhow::anyhow!("Invalid address format: {}", e))?;
        
        let utxos_response = self.client
            .get_utxos_by_addresses(vec![rpc_address])
            .await
            .map_err(|e| {
                tracing::error!("RPC Error: {}", e);
                anyhow::anyhow!("Failed to get address UTXOs: {}", e)
            })?;

        // Convert gRPC response to simplified format: (txid, output_index, amount)
        let mut result = Vec::new();
        for entry in utxos_response {
            // entry.utxo_entry is an RpcUtxoEntry, not Option
            result.push((
                entry.outpoint.transaction_id.to_string(),
                entry.outpoint.index,
                entry.utxo_entry.amount,
            ));
        }

        Ok(result)
    }

    /// Get block by hash
    pub async fn get_block(&self, hash: &str) -> Result<String> {
        tracing::debug!("RPC Call: GetBlock({})", hash);
        
        // Parse hex string to bytes for RpcHash
        let hash_bytes = hex::decode(hash)
            .map_err(|e| anyhow::anyhow!("Invalid hash format: {}", e))?;
        
        if hash_bytes.len() != 32 {
            return Err(anyhow::anyhow!("Hash must be 32 bytes"));
        }
        
        let mut hash_array = [0u8; 32];
        hash_array.copy_from_slice(&hash_bytes);
        
        let block = self.client
            .get_block(hash_array.into(), true)
            .await
            .map_err(|e| {
                tracing::error!("RPC Error: {}", e);
                anyhow::anyhow!("Failed to get block: {}", e)
            })?;

        Ok(serde_json::to_string(&block)
            .unwrap_or_else(|_| "{}".to_string()))
    }

    /// Get block count
    #[allow(dead_code)]
    pub async fn get_block_count(&self) -> Result<u64> {
        tracing::debug!("RPC Call: GetBlockCount");
        
        let response = self.client
            .get_block_count()
            .await
            .map_err(|e| {
                tracing::error!("RPC Error: {}", e);
                anyhow::anyhow!("Failed to get block count: {}", e)
            })?;

        Ok(response.block_count)
    }

    /// Get mempool entries (transactions in mempool)
    pub async fn get_mempool_entries(&self) -> Result<Vec<String>> {
        tracing::debug!("RPC Call: GetMempoolEntries");
        
        let entries = self.client
            .get_mempool_entries(false, false)
            .await
            .map_err(|e| {
                tracing::error!("RPC Error: {}", e);
                anyhow::anyhow!("Failed to get mempool entries: {}", e)
            })?;

        // RpcTransaction implements Debug, convert to string via debug format
        Ok(entries.iter().map(|e| format!("{:?}", e.transaction)).collect())
    }

    /// Disconnect from the node
    #[allow(dead_code)]
    pub async fn disconnect(&self) -> Result<()> {
        tracing::debug!("Disconnecting from node");
        
        self.client
            .disconnect()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to disconnect: {}", e))
    }
}
