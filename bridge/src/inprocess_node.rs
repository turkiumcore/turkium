use std::sync::Arc;

use turkium_core::signals::Shutdown;
use turkium_utils::fd_budget;
use turkiumd_lib::{args as turkiumd_args, daemon as turkiumd_daemon};

pub(crate) struct InProcessNode {
    core: Arc<turkium_core::core::Core>,
    workers: Vec<std::thread::JoinHandle<()>>,
}

impl InProcessNode {
    pub(crate) fn start_from_args(args: turkiumd_args::Args) -> Result<Self, anyhow::Error> {
        let _ = fd_budget::try_set_fd_limit(turkiumd_daemon::DESIRED_DAEMON_SOFT_FD_LIMIT);

        let runtime = turkiumd_daemon::Runtime::from_args(&args);
        let fd_total_budget =
            fd_budget::limit() - args.rpc_max_clients as i32 - args.inbound_limit as i32 - args.outbound_target as i32;
        let (core, _) = turkiumd_daemon::create_core_with_runtime(&runtime, &args, fd_total_budget);
        let workers = core.start();
        Ok(Self { core, workers })
    }

    fn shutdown(self) {
        self.core.shutdown();
        self.core.join(self.workers);
    }
}

pub(crate) async fn shutdown_inprocess(node: InProcessNode) {
    let _ = tokio::task::spawn_blocking(move || node.shutdown()).await;
}
