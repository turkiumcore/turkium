use crate::config::Config;
use crate::peer_manager::PeerManager;
use anyhow::Result;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::Arc;
use trust_dns_proto::op::{Message, MessageType, OpCode, ResponseCode};
use trust_dns_proto::rr::{RecordType, RData};
use trust_dns_proto::serialize::binary::*;

pub struct DnsServer {
    peer_manager: Arc<PeerManager>,
    config: Config,
}

impl DnsServer {
    pub fn new(peer_manager: Arc<PeerManager>, config: Config) -> Self {
        Self {
            peer_manager,
            config,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let addr = format!("{}:{}", self.config.dns_listen, self.config.dns_port);
        let socket = UdpSocket::bind(&addr)?;
        socket.set_nonblocking(true)?;

        tracing::info!("DNS server listening on {}", addr);

        let socket = Arc::new(socket);

        loop {
            let mut buffer = vec![0u8; 512];

            match socket.recv_from(&mut buffer) {
                Ok((size, src_addr)) => {
                    let socket_clone = socket.clone();
                    let peer_manager = self.peer_manager.clone();
                    let hostname = self.config.hostname.clone();

                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_query(
                            &socket_clone,
                            src_addr,
                            &buffer[..size],
                            &peer_manager,
                            &hostname,
                        ) {
                            tracing::debug!("Error handling DNS query: {}", e);
                        }
                    });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }
                Err(e) => {
                    tracing::error!("DNS socket error: {}", e);
                }
            }
        }
    }

    fn handle_query(
        socket: &UdpSocket,
        src_addr: SocketAddr,
        buffer: &[u8],
        peer_manager: &Arc<PeerManager>,
        hostname: &str,
    ) -> Result<()> {
        // Parse incoming DNS query
        let mut decoder = BinDecoder::new(buffer);
        let message = Message::from_bytes(&mut decoder)?;

        // Build response
        let mut response = Message::new();
        response.set_id(message.id());
        response.set_message_type(MessageType::Response);
        response.set_op_code(OpCode::Query);
        response.set_response_code(ResponseCode::NoError);

        // Process queries
        for query in message.queries() {
            let qname = query.name();
            let qtype = query.query_type();

            tracing::debug!("DNS query: {} {:?}", qname, qtype);

            // Check if query is for our seeder hostname
            if qname.to_utf8().to_lowercase() == format!("{}.", hostname.to_lowercase()) {
                if qtype == RecordType::A {
                    // Get healthy peer IPs
                    let ips = peer_manager.get_ips_for_dns();

                    if !ips.is_empty() {
                        // Add A records for each peer IP
                        for ip in ips.iter().take(16) {
                            // Limit to 16 responses per query
                            if let IpAddr::V4(ipv4) = ip {
                                let rdata = RData::A(*ipv4);
                                response.add_answer(trust_dns_proto::rr::Record::from_rdata(
                                    qname.clone(),
                                    300, // TTL
                                    rdata,
                                ));
                            }
                        }

                        tracing::debug!(
                            "Responding with {} A records for {}",
                            response.answer_count(),
                            hostname
                        );
                    } else {
                        response.set_response_code(ResponseCode::ServFail);
                        tracing::warn!("No healthy peers available for DNS query");
                    }
                }
            } else {
                response.set_response_code(ResponseCode::NXDomain);
            }
        }

        // Serialize and send response
        let mut encoder = BinEncoder::new(Vec::new());
        response.emit(&mut encoder)?;
        let response_bytes = encoder.into_bytes();

        socket.send_to(&response_bytes, src_addr)?;

        Ok(())
    }
}
