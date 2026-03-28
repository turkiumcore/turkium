use crate::notification::Notification;
use Turkium_notify::{connection::ChannelConnection, notifier::Notifier};

pub type ConsensusNotifier = Notifier<Notification, ChannelConnection<Notification>>;
