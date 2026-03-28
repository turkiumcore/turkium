use Turkium_notify::{collector::CollectorFrom, converter::ConverterFrom};
use Turkium_rpc_core::Notification;

pub type WrpcServiceConverter = ConverterFrom<Notification, Notification>;
pub type WrpcServiceCollector = CollectorFrom<WrpcServiceConverter>;
