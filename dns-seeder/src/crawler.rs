use crate::config::Config;
use crate::peer_manager::PeerManager;
use anyhow::Result;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::sleep;

pub struct Crawler {
    peer_manager: Arc<PeerManager>,
    config: Config,
}

impl Crawler {
    pub fn new(peer_manager: Arc<PeerManager>, config: Config) -> Self {
        Self {
            peer_manager,
            config,
        }
    }

    pub async fn run(&self) -> Result<()> {
        tracing::info!("Crawler started");

        // Bootstrap from initial peer if provided
        if let Some(bootstrap) = &self.config.bootstrap_peer {
            self.bootstrap_from_peer(bootstrap).await;
        }

        loop {
            // Cleanup expired peers
            self.peer_manager.cleanup_expired_peers();

            // Get stats
            let stats = self.peer_manager.stats();
            tracing::info!(
                "Peer stats - Total: {}, Healthy: {}, Unhealthy: {}",
                stats.total_peers,
                stats.healthy_peers,
                stats.unhealthy_peers
            );

            // Get peers to crawl
            let peers_to_crawl = self.peer_manager.get_peers_for_crawl();

            if peers_to_crawl.is_empty() && stats.healthy_peers < self.config.min_peers {
                tracing::warn!("Not enough peers, attempting to bootstrap");
                if let Some(bootstrap) = &self.config.bootstrap_peer {
                    self.bootstrap_from_peer(bootstrap).await;
                }
            }

            // Crawl peers in parallel
            let mut handles = vec![];
            for peer in peers_to_crawl.iter().take(10) {
                let peer_manager = self.peer_manager.clone();
                let peer_info = peer.clone();

                let handle = tokio::spawn(async move {
                    let addr = SocketAddr::new(peer_info.ip, peer_info.port);
                    match Self::crawl_peer(addr).await {
                        Ok(new_peers) => {
                            for new_peer in new_peers {
                                peer_manager.add_peer(new_peer.0, new_peer.1);
                            }
                            peer_manager.update_peer(peer_info.ip, peer_info.port, None);
                        }
                        Err(e) => {
                            tracing::debug!("Failed to crawl peer {}: {}", addr, e);
                            peer_manager.mark_peer_unhealthy(peer_info.ip, peer_info.port);
                        }
                    }
                });

                handles.push(handle);
            }

            // Wait for all crawl tasks
            for handle in handles {
                let _ = handle.await;
            }

            // Sleep before next crawl
            sleep(Duration::from_secs(self.config.crawl_interval)).await;
        }
    }

    async fn bootstrap_from_peer(&self, peer_addr: &str) -> bool {
        match SocketAddr::from_str(peer_addr) {
            Ok(addr) => {
                tracing::info!("Bootstrapping from {}", addr);
                match Self::crawl_peer(addr).await {
                    Ok(peers) => {
                        for (ip, port) in peers {
                            self.peer_manager.add_peer(ip, port);
                        }
                        tracing::info!("Bootstrap successful, discovered {} peers", self.peer_manager.get_peer_count());
                        true
                    }
                    Err(e) => {
                        tracing::error!("Bootstrap failed: {}", e);
                        false
                    }
                }
            }
            Err(e) => {
                tracing::error!("Invalid bootstrap peer address: {}", e);
                false
            }
        }
    }

    async fn crawl_peer(addr: SocketAddr) -> Result<Vec<(IpAddr, u16)>> {
        let mut stream = tokio::time::timeout(Duration::from_secs(5), TcpStream::connect(addr))
            .await??;

        // Send version message (simplified turkium P2P protocol)
        // This is a basic handshake - in production, use proper Turkium protocol
        let version_msg = b"VERSION\n";
        stream.write_all(version_msg).await?;

        // Read response with timeout
        let mut buffer = vec![0u8; 4096];
        let n = tokio::time::timeout(Duration::from_secs(5), stream.read(&mut buffer))
            .await??;

        if n == 0 {
            return Ok(vec![]);
        }

        // Parse peer list from response
        // This is a simplified parser - in production, use proper Turkium protocol parsing
        let response = String::from_utf8_lossy(&buffer[..n]);
        let peers = Self::parse_peer_list(&response);

        Ok(peers)
    }

    fn parse_peer_list(response: &str) -> Vec<(IpAddr, u16)> {
        let mut peers = vec![];

        for line in response.lines() {
            if let Some((ip_str, port_str)) = line.split_once(':') {
                if let (Ok(ip), Ok(port)) = (ip_str.parse::<IpAddr>(), port_str.parse::<u16>()) {
                    peers.push((ip, port));
                }
            }
        }

        peers
    }
}
