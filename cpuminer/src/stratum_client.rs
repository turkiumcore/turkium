use crate::Error;
use log::{debug, error, info, warn};
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpStream, tcp::{OwnedReadHalf, OwnedWriteHalf}};
use tokio::sync::mpsc::{self, Sender};

#[derive(Debug, Clone)]
pub struct BlockTemplate {
    pub job_id: String,
    #[allow(dead_code)]
    pub prev_hash: String,
    #[allow(dead_code)]
    pub coinbase1: String,
    #[allow(dead_code)]
    pub coinbase2: String,
    #[allow(dead_code)]
    pub merkle_branch: Vec<String>,
    #[allow(dead_code)]
    pub version: String,
    #[allow(dead_code)]
    pub nbits: String,
    #[allow(dead_code)]
    pub ntime: String,
}

pub struct StratumClient {
    writer: OwnedWriteHalf,
    reader: BufReader<OwnedReadHalf>,
    miner_address: String,
    request_id: u64,
    pub template_tx: Sender<BlockTemplate>,
}

impl StratumClient {
    pub async fn connect(
        address: &str,
        miner_address: String,
    ) -> Result<(Self, tokio::sync::mpsc::Receiver<BlockTemplate>), Error> {
        let stream = TcpStream::connect(address).await?;
        let (reader_stream, writer_stream) = stream.into_split();
        let reader = BufReader::new(reader_stream);

        let (template_tx, template_rx) = mpsc::channel(10);

        let mut client = Self {
            writer: writer_stream,
            reader,
            miner_address,
            request_id: 1,
            template_tx,
        };

        // Subscribe
        client.subscribe().await?;

        // Authorize
        client.authorize().await?;

        info!("Connected to Stratum pool at {}", address);

        Ok((client, template_rx))
    }

    async fn subscribe(&mut self) -> Result<(), Error> {
        let req = json!({
            "id": self.request_id,
            "method": "mining.subscribe",
            "params": ["turkium-miner/0.1.0"]
        });

        self.send_request(&req).await?;
        self.request_id += 1;

        // Read response
        let response = self.read_response().await?;
        debug!("Subscribe response: {:?}", response);

        Ok(())
    }

    async fn authorize(&mut self) -> Result<(), Error> {
        let req = json!({
            "id": self.request_id,
            "method": "mining.authorize",
            "params": [&self.miner_address, ""]
        });

        self.send_request(&req).await?;
        self.request_id += 1;

        // Read response
        let response = self.read_response().await?;
        debug!("Authorize response: {:?}", response);

        Ok(())
    }

    async fn send_request(&mut self, req: &Value) -> Result<(), Error> {
        let json_str = req.to_string();
        self.writer.write_all(json_str.as_bytes()).await?;
        self.writer.write_all(b"\n").await?;
        self.writer.flush().await?;
        Ok(())
    }

    async fn read_response(&mut self) -> Result<Value, Error> {
        let mut line = String::new();
        self.reader.read_line(&mut line).await?;

        if line.is_empty() {
            return Err("Connection closed".into());
        }

        let response: Value = serde_json::from_str(&line)?;
        Ok(response)
    }

    pub async fn listen_for_notifications(&mut self) -> Result<(), Error> {
        loop {
            let mut line = String::new();
            self.reader.read_line(&mut line).await?;

            if line.is_empty() {
                error!("Connection closed by pool");
                break;
            }

            match serde_json::from_str::<Value>(&line) {
                Ok(msg) => {
                    if let Some(method) = msg.get("method").and_then(|v| v.as_str()) {
                        match method {
                            "mining.notify" => {
                                if let Err(e) = self.handle_notify(&msg).await {
                                    warn!("Error handling notify: {}", e);
                                }
                            }
                            "mining.set_difficulty" => {
                                if let Some(params) = msg.get("params").and_then(|v| v.as_array()) {
                                    if let Some(difficulty) = params.get(0).and_then(|v| v.as_f64()) {
                                        debug!("Pool set difficulty: {}", difficulty);
                                    }
                                }
                            }
                            _ => debug!("Unknown method: {}", method),
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to parse message: {}", e);
                }
            }
        }

        Ok(())
    }

    async fn handle_notify(&mut self, msg: &Value) -> Result<(), Error> {
        let params = msg
            .get("params")
            .and_then(|v| v.as_array())
            .ok_or("Invalid notify params")?;

        if params.len() < 9 {
            return Err("Not enough params in notify".into());
        }

        let job_id = params[0].as_str().ok_or("Invalid job_id")?;
        let prev_hash = params[1].as_str().ok_or("Invalid prev_hash")?;
        let coinbase1 = params[2].as_str().ok_or("Invalid coinbase1")?;
        let coinbase2 = params[3].as_str().ok_or("Invalid coinbase2")?;
        let merkle_branch = params[4]
            .as_array()
            .ok_or("Invalid merkle_branch")?
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        let version = params[5].as_str().ok_or("Invalid version")?;
        let nbits = params[6].as_str().ok_or("Invalid nbits")?;
        let ntime = params[7].as_str().ok_or("Invalid ntime")?;

        let template = BlockTemplate {
            job_id: job_id.to_string(),
            prev_hash: prev_hash.to_string(),
            coinbase1: coinbase1.to_string(),
            coinbase2: coinbase2.to_string(),
            merkle_branch,
            version: version.to_string(),
            nbits: nbits.to_string(),
            ntime: ntime.to_string(),
        };

        debug!("Received block template: job_id={}", template.job_id);

        // Send to miner
        self.template_tx.send(template).await?;

        Ok(())
    }
}
