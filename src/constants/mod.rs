use std::time::Duration;

pub mod auth_plugin_names;
pub mod capability_flags;
pub mod checksum_type;
pub mod column_type;
pub mod database_provider;

///Packet Constants
pub const PACKET_HEADER_SIZE: usize = 4;
pub const MAX_BODY_LENGTH: usize = 16777215;
pub const NULL_TERMINATOR: u8 = 0;
pub const UTF8_MB4_GENERAL_CI: u8 = 45;

///Event Constants
pub const EVENT_HEADER_SIZE: usize = 19;
pub const PAYLOAD_BUFFER_SIZE: usize = 32 * 1024;
pub const FIRST_EVENT_POSITION: usize = 4;

/// Timeout constants
/// Takes into account network latency.
pub const TIMEOUT_DELTA: Duration = Duration::from_secs(1);
pub const TIMEOUT_MESSAGE: &str =
    "Could not receive a master heartbeat within the specified interval";
