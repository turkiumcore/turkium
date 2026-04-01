use turkium_notify::{collector::CollectorFrom, converter::ConverterFrom};
use turkium_rpc_core::Notification;

pub type GrpcServiceConverter = ConverterFrom<Notification, Notification>;
pub type GrpcServiceCollector = CollectorFrom<GrpcServiceConverter>;
