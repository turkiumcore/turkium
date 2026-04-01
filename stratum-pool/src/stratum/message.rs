use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StratumRequest {
    pub id: u64,
    pub method: String,
    pub params: Vec<Value>,
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
    },
    Submit {
        username: String,
        job_id: String,
        nonce: String,
        result: String,
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
                
                Ok(StratumMessage::Authorize {
                    id: req.id,
                    username,
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
                    username,
                    job_id,
                    nonce,
                    result,
                })
            }
            _ => Err(format!("unknown method: {}", req.method)),
        }
    }
}
