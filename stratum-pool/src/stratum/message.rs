use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StratumRequest {
    pub id: u64,
    pub method: String,
    pub params: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StratumResponse {
    pub id: u64,
    pub result: Option<Value>,
    pub error: Option<Value>,
}

#[derive(Debug, Clone)]
pub enum StratumMessage {
    // Client -> Server
    Subscribe {
        id: u64,
        user_agent: String,
    },
    Authorize {
        id: u64,
        username: String,
        password: String,
    },
    Submit {
        id: u64,
        username: String,
        job_id: String,
        nonce: String,
        result: String,
    },
    
    // Server -> Client
    Notify {
        job_id: String,
        prev_hash: String,
        coinbase1: String,
        coinbase2: String,
        merkle_branch: Vec<String>,
        version: String,
        nbits: String,
        ntime: String,
        clean_jobs: bool,
    },
    SetDifficulty {
        difficulty: f64,
    },
    SetExtraNonce {
        extra_nonce1: String,
        extra_nonce2_size: u32,
    },
}

impl StratumMessage {
    pub fn parse(req: &StratumRequest) -> Result<Self, String> {
        match req.method.as_str() {
            "mining.subscribe" => {
                let user_agent = req.params.get(0)
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                Ok(StratumMessage::Subscribe {
                    id: req.id,
                    user_agent,
                })
            }
            "mining.authorize" => {
                let username = req.params.get(0)
                    .and_then(|v| v.as_str())
                    .ok_or("missing username")?
                    .to_string();
                
                let password = req.params.get(1)
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                
                Ok(StratumMessage::Authorize {
                    id: req.id,
                    username,
                    password,
                })
            }
            "mining.submit" => {
                let username = req.params.get(0)
                    .and_then(|v| v.as_str())
                    .ok_or("missing username")?
                    .to_string();
                
                let job_id = req.params.get(1)
                    .and_then(|v| v.as_str())
                    .ok_or("missing job_id")?
                    .to_string();
                
                let nonce = req.params.get(2)
                    .and_then(|v| v.as_str())
                    .ok_or("missing nonce")?
                    .to_string();
                
                let result = req.params.get(3)
                    .and_then(|v| v.as_str())
                    .ok_or("missing result")?
                    .to_string();
                
                Ok(StratumMessage::Submit {
                    id: req.id,
                    username,
                    job_id,
                    nonce,
                    result,
                })
            }
            _ => Err(format!("unknown method: {}", req.method)),
        }
    }

    pub fn to_response(&self) -> StratumResponse {
        match self {
            StratumMessage::Subscribe { id, .. } => {
                StratumResponse {
                    id: *id,
                    result: Some(json!([
                        [["mining.set_difficulty", "1"], ["mining.notify", "1"]],
                        "00000000",
                        2
                    ])),
                    error: None,
                }
            }
            StratumMessage::Authorize { id, .. } => {
                StratumResponse {
                    id: *id,
                    result: Some(json!(true)),
                    error: None,
                }
            }
            StratumMessage::Submit { id, .. } => {
                StratumResponse {
                    id: *id,
                    result: Some(json!(true)),
                    error: None,
                }
            }
            _ => {
                StratumResponse {
                    id: 0,
                    result: None,
                    error: Some(json!("invalid message")),
                }
            }
        }
    }

    pub fn to_notify_json(&self) -> Option<String> {
        match self {
            StratumMessage::Notify {
                job_id,
                prev_hash,
                coinbase1,
                coinbase2,
                merkle_branch,
                version,
                nbits,
                ntime,
                clean_jobs,
            } => {
                let msg = json!({
                    "jsonrpc": "2.0",
                    "method": "mining.notify",
                    "params": [
                        job_id,
                        prev_hash,
                        coinbase1,
                        coinbase2,
                        merkle_branch,
                        version,
                        nbits,
                        ntime,
                        clean_jobs
                    ]
                });
                Some(msg.to_string())
            }
            StratumMessage::SetDifficulty { difficulty } => {
                let msg = json!({
                    "jsonrpc": "2.0",
                    "method": "mining.set_difficulty",
                    "params": [difficulty]
                });
                Some(msg.to_string())
            }
            StratumMessage::SetExtraNonce { extra_nonce1, extra_nonce2_size } => {
                let msg = json!({
                    "jsonrpc": "2.0",
                    "method": "mining.set_extranonce",
                    "params": [extra_nonce1, extra_nonce2_size]
                });
                Some(msg.to_string())
            }
            _ => None,
        }
    }
}
