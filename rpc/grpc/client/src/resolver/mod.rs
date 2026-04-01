use super::error::Result;
use turkium_grpc_core::{
    ops::TurkiumdPayloadOps,
    protowire::{TurkiumdRequest, TurkiumdResponse},
};
use core::fmt::Debug;
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;

pub(crate) mod id;
pub(crate) mod matcher;
pub(crate) mod queue;

pub(crate) trait Resolver: Send + Sync + Debug {
    fn register_request(&self, op: TurkiumdPayloadOps, request: &TurkiumdRequest) -> TurkiumdResponseReceiver;
    fn handle_response(&self, response: TurkiumdResponse);
    fn remove_expired_requests(&self, timeout: Duration);
}

pub(crate) type DynResolver = Arc<dyn Resolver>;

pub(crate) type TurkiumdResponseSender = oneshot::Sender<Result<TurkiumdResponse>>;
pub(crate) type TurkiumdResponseReceiver = oneshot::Receiver<Result<TurkiumdResponse>>;
