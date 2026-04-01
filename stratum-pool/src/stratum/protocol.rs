use bytes::{BytesMut, BufMut};
use serde_json::from_slice;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use super::message::StratumRequest;

#[derive(Debug)]
pub enum StratumError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    ParseError(String),
}

impl std::fmt::Display for StratumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StratumError::IoError(e) => write!(f, "IO error: {}", e),
            StratumError::JsonError(e) => write!(f, "JSON error: {}", e),
            StratumError::ParseError(s) => write!(f, "Parse error: {}", s),
        }
    }
}

impl std::error::Error for StratumError {}

impl From<std::io::Error> for StratumError {
    fn from(e: std::io::Error) -> Self {
        StratumError::IoError(e)
    }
}

impl From<serde_json::Error> for StratumError {
    fn from(e: serde_json::Error) -> Self {
        StratumError::JsonError(e)
    }
}

pub struct StratumCodec;

impl StratumCodec {
    pub async fn read_message(stream: &mut TcpStream) -> Result<StratumRequest, StratumError> {
        let mut buffer = BytesMut::with_capacity(4096);
        let mut temp = [0u8; 1024];

        loop {
            let n = stream.read(&mut temp).await?;
            if n == 0 {
                return Err(StratumError::ParseError("connection closed".to_string()));
            }

            buffer.put_slice(&temp[..n]);

            // Look for newline
            if let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                let line = buffer.split_to(pos + 1);
                let json_str = String::from_utf8_lossy(&line[..line.len() - 1]);
                
                match from_slice::<StratumRequest>(json_str.as_bytes()) {
                    Ok(req) => return Ok(req),
                    Err(e) => return Err(StratumError::JsonError(e)),
                }
            }

            // Prevent buffer from growing too large
            if buffer.len() > 65536 {
                return Err(StratumError::ParseError("message too large".to_string()));
            }
        }
    }
}
