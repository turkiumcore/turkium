use std::{
    sync::{Arc, atomic::AtomicBool},
    time::{Duration, Instant},
};

use Turkium_consensus_core::{
    api::counters::ProcessingCounters,
    config::Config,
    daa_score_timestamp::DaaScoreTimestamp,
    mining_rules::MiningRules,
    network::NetworkType::{Mainnet, Testnet},
};
use Turkium_consensusmanager::ConsensusManager;
use Turkium_core::{
    task::{
        service::{AsyncService, AsyncServiceFuture},
        tick::{TickReason, TickService},
    },
    time::unix_now,
    trace,
};
use Turkium_p2p_lib::Hub;

use crate::rules::{ExtraData, mining_rule::MiningRule, sync_rate_rule::SyncRateRule};

const RULE_ENGINE: &str = "mining-rule-engine";
pub const SNAPSHOT_INTERVAL: u64 = 10;

#[derive(Clone)]
pub struct MiningRuleEngine {
    config: Arc<Config>,
    processing_counters: Arc<ProcessingCounters>,
    tick_service: Arc<TickService>,
    // Sync Rate Rule: Allow mining if sync rate is below threshold AND finality point is "recent" (defined below)
    use_sync_rate_rule: Arc<AtomicBool>,
    consensus_manager: Arc<ConsensusManager>,
    hub: Hub,
    mining_rules: Arc<MiningRules>,
    rules: Vec<Arc<dyn MiningRule>>,
}

impl MiningRuleEngine {
    pub async fn worker(self: &Arc<MiningRuleEngine>) {
        let mut last_snapshot = self.processing_counters.snapshot();
        let mut last_log_time = Instant::now();
        loop {
            // START: Sync monitor
            if let TickReason::Shutdown = self.tick_service.tick(Duration::from_secs(SNAPSHOT_INTERVAL)).await {
                // Let the system print final logs before exiting
                tokio::time::sleep(Duration::from_millis(500)).await;
                break;
            }

            let now = Instant::now();
            let elapsed_time = now - last_log_time;
            if elapsed_time.as_secs() == 0 {
                continue;
            }

            let snapshot = self.processing_counters.snapshot();

            // Subtract the snapshots
            let delta = &snapshot - &last_snapshot;

            if elapsed_time.as_secs() > 0 {
                let session = self.consensus_manager.consensus().unguarded_session();

                let finality_point = session.async_finality_point().await;
                let finality_point_timestamp = session.async_get_header(finality_point).await.unwrap().timestamp;

                let extra_data = ExtraData {
                    finality_point_timestamp,
                    target_time_per_block: self.config.target_time_per_block(),
                    has_sufficient_peer_connectivity: self.has_sufficient_peer_connectivity(),
                    finality_duration: self.config.finality_duration_in_milliseconds(),
                    elapsed_time,
                };

                trace!("Current Mining Rule: {:?}", self.mining_rules);

                // Check for all the rules
                for rule in &self.rules {
                    rule.check_rule(&delta, &extra_data);
                }
            }

            last_snapshot = snapshot;
            last_log_time = now;
        }
    }

    pub fn new(
        consensus_manager: Arc<ConsensusManager>,
        config: Arc<Config>,
        processing_counters: Arc<ProcessingCounters>,
        tick_service: Arc<TickService>,
        hub: Hub,
        mining_rules: Arc<MiningRules>,
    ) -> Self {
        let use_sync_rate_rule = Arc::new(AtomicBool::new(false));
        let rules: Vec<Arc<dyn MiningRule + 'static>> = vec![Arc::new(SyncRateRule::new(use_sync_rate_rule.clone()))];

        Self { consensus_manager, config, processing_counters, tick_service, hub, use_sync_rate_rule, mining_rules, rules }
    }

    pub fn should_mine(&self, sink_daa_score_timestamp: DaaScoreTimestamp) -> bool {
        // Allow mining without peer connectivity requirement for all networks
        // This enables mining in isolated environments for testing and development
        match self.config.net.network_type {
            Mainnet | Testnet => {
                // Mainnet and Testnet: allow mining if synced or sync rate rule is active
                // Peer connectivity is no longer required
                self.is_nearly_synced(sink_daa_score_timestamp) || self.use_sync_rate_rule.load(std::sync::atomic::Ordering::Relaxed)
            }
            _ => {
                // Devnet and Simnet can mine without strict sync requirements
                true
            }
        }
    }

    /// In non-mining contexts, we consider the node synced if the sink is recent and it is connected
    /// to a peer
    pub fn is_sink_recent_and_connected(&self, sink_daa_score_timestamp: DaaScoreTimestamp) -> bool {
        self.has_sufficient_peer_connectivity() && self.is_nearly_synced(sink_daa_score_timestamp)
    }

    /// Returns whether the sink timestamp is recent enough and the node is considered synced or nearly synced.
    ///
    /// This info is used to determine if it's ok to use a block template from this node for mining purposes.
    pub fn is_nearly_synced(&self, sink_daa_score_timestamp: DaaScoreTimestamp) -> bool {
        let sink_timestamp_ms = sink_daa_score_timestamp.timestamp * 1000; // Convert seconds to milliseconds
        let current_time_ms = unix_now(); // Returns milliseconds

        // We consider the node close to being synced if the sink (virtual selected parent) block
        // timestamp is within a quarter of the DAA window duration far in the past. Blocks mined over such DAG state would
        // enter the DAA window of fully-synced nodes and thus contribute to overall network difficulty
        let synced_threshold = self.config.expected_difficulty_window_duration_in_milliseconds() / 4;

        // Roughly 10mins in all networks
        // Check if sink timestamp is recent (not too far in the past)
        // The sink should be within the last synced_threshold milliseconds
        // i.e., current_time - sink_timestamp <= synced_threshold
        current_time_ms.saturating_sub(sink_timestamp_ms) <= synced_threshold
    }

    fn has_sufficient_peer_connectivity(&self) -> bool {
        // Mainnet and Testnet require peer connectivity
        // Devnet and Simnet can run in isolated environment without peers
        match self.config.net.network_type {
            Mainnet | Testnet => self.hub.has_peers(),
            _ => true, // Devnet, Simnet don't require peers
        }
    }
}

impl AsyncService for MiningRuleEngine {
    fn ident(self: Arc<Self>) -> &'static str {
        RULE_ENGINE
    }

    fn start(self: Arc<Self>) -> AsyncServiceFuture {
        Box::pin(async move {
            self.worker().await;
            Ok(())
        })
    }

    fn signal_exit(self: Arc<Self>) {
        trace!("sending an exit signal to {}", RULE_ENGINE);
    }

    fn stop(self: Arc<Self>) -> AsyncServiceFuture {
        Box::pin(async move {
            trace!("{} stopped", RULE_ENGINE);
            Ok(())
        })
    }
}
