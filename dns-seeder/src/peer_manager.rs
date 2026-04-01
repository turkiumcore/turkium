use crate::config::Config;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::net::IpAddr;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct PeerInfo {
    pub ip: IpAddr,
    pub port: u16,
    pub last_seen: DateTime<Utc>,
    pub last_crawled: Option<DateTime<Utc>>,
    pub version: Option<String>,
    pub is_healthy: bool,
}

impl PeerInfo {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self {
            ip,
            port,
            last_seen: Utc::now(),
            last_crawled: None,
            version: None,
            is_healthy: true,
        }
    }

    pub fn is_expired(&self, timeout_secs: u64) -> bool {
        let elapsed = Utc::now()
            .signed_duration_since(self.last_seen)
            .num_seconds() as u64;
        elapsed > timeout_secs
    }

    pub fn update_seen(&mut self) {
        self.last_seen = Utc::now();
    }

    pub fn update_crawled(&mut self, version: Option<String>) {
        self.last_crawled = Some(Utc::now());
        self.version = version;
        self.is_healthy = true;
    }

    pub fn mark_unhealthy(&mut self) {
        self.is_healthy = false;
    }
}

pub struct PeerManager {
    peers: DashMap<String, PeerInfo>,
    config: Config,
}

impl PeerManager {
    pub fn new(config: Config) -> Self {
        Self {
            peers: DashMap::new(),
            config,
        }
    }

    pub fn add_peer(&self, ip: IpAddr, port: u16) -> bool {
        let key = format!("{}:{}", ip, port);

        if self.peers.len() >= self.config.max_peers {
            return false;
        }

        if !self.peers.contains_key(&key) {
            self.peers.insert(key, PeerInfo::new(ip, port));
            tracing::debug!("Added peer: {}:{}", ip, port);
            return true;
        }

        false
    }

    pub fn update_peer(&self, ip: IpAddr, port: u16, version: Option<String>) {
        let key = format!("{}:{}", ip, port);

        if let Some(mut peer) = self.peers.get_mut(&key) {
            peer.update_crawled(version);
        } else {
            self.add_peer(ip, port);
        }
    }

    pub fn mark_peer_unhealthy(&self, ip: IpAddr, port: u16) {
        let key = format!("{}:{}", ip, port);
        if let Some(mut peer) = self.peers.get_mut(&key) {
            peer.mark_unhealthy();
        }
    }

    pub fn get_healthy_peers(&self) -> Vec<PeerInfo> {
        self.peers
            .iter()
            .filter(|entry| {
                let peer = entry.value();
                peer.is_healthy && !peer.is_expired(self.config.peer_timeout)
            })
            .map(|entry| entry.value().clone())
            .collect()
    }

    pub fn get_all_peers(&self) -> Vec<PeerInfo> {
        self.peers
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    pub fn get_peers_for_crawl(&self) -> Vec<PeerInfo> {
        self.peers
            .iter()
            .filter(|entry| {
                let peer = entry.value();
                peer.is_healthy
                    && !peer.is_expired(self.config.peer_timeout)
                    && (peer.last_crawled.is_none()
                        || Utc::now()
                            .signed_duration_since(peer.last_crawled.unwrap())
                            .num_secs() > 3600)
            })
            .map(|entry| entry.value().clone())
            .collect()
    }

    pub fn cleanup_expired_peers(&self) {
        let expired_count = self.peers.len();

        self.peers.retain(|_, peer| !peer.is_expired(self.config.peer_timeout));

        let removed = expired_count - self.peers.len();
        if removed > 0 {
            tracing::info!("Cleaned up {} expired peers", removed);
        }
    }

    pub fn get_peer_count(&self) -> usize {
        self.peers.len()
    }

    pub fn get_healthy_peer_count(&self) -> usize {
        self.get_healthy_peers().len()
    }

    pub fn get_random_peers(&self, count: usize) -> Vec<PeerInfo> {
        use rand::seq::SliceRandom;

        let mut peers = self.get_healthy_peers();
        let mut rng = rand::thread_rng();
        peers.shuffle(&mut rng);
        peers.into_iter().take(count).collect()
    }

    pub fn get_ips_for_dns(&self) -> Vec<IpAddr> {
        self.get_healthy_peers()
            .into_iter()
            .map(|p| p.ip)
            .collect()
    }

    pub fn stats(&self) -> PeerStats {
        let all_peers = self.get_all_peers();
        let healthy_peers = self.get_healthy_peers();

        PeerStats {
            total_peers: all_peers.len(),
            healthy_peers: healthy_peers.len(),
            unhealthy_peers: all_peers.len() - healthy_peers.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PeerStats {
    pub total_peers: usize,
    pub healthy_peers: usize,
    pub unhealthy_peers: usize,
}
