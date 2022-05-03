use std::sync::Arc;

pub mod dispatcher;
pub mod dns_client;
pub mod inbound;
pub mod logger;
pub mod nat_manager;
pub mod outbound;
pub mod router;

pub mod copy;
#[cfg(any(
    target_os = "ios",
    target_os = "android",
    target_os = "macos",
    target_os = "linux",
    target_os = "windows"
))]
pub mod fake_dns;

pub type SyncDnsClient = Arc<dns_client::DnsClient>;
