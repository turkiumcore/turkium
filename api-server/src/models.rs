use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ============================================
// Transaction Models
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionCount {
    pub timestamp: u64,
    pub date_time: DateTime<Utc>,
    pub coinbase: u64,
    pub regular: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTransactionsRequest {
    pub transaction_ids: Vec<String>,
}

// ============================================
// Address Models
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBalance {
    pub address: String,
    pub balance: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressTxCount {
    pub address: String,
    pub transaction_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXO {
    pub transaction_id: String,
    pub output_index: u32,
    pub amount: u64,
    pub script_public_key: String,
    pub block_daa_score: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressTransactions {
    pub address: String,
    pub transactions: Vec<serde_json::Value>,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressName {
    pub address: String,
    pub name: Option<String>,
}

// ============================================
// Health Check
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub node_connected: bool,
}
