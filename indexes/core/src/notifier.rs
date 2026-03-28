use crate::notification::Notification;
use Turkium_notify::{connection::ChannelConnection, notifier::Notifier};

pub type IndexNotifier = Notifier<Notification, ChannelConnection<Notification>>;
