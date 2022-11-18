use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Gtid {
    /// Gets domain identifier in multi-master setup.
    pub domain_id: u32,

    /// Gets identifier of the server that generated the event.
    pub server_id: u32,

    /// Gets sequence number of the event on the original server.
    pub sequence: u64,
}

impl Gtid {
    pub fn new(domain_id: u32, server_id: u32, sequence: u64) -> Self {
        Self {
            domain_id,
            server_id,
            sequence,
        }
    }
}

impl fmt::Display for Gtid {
    /// Returns string representation of Gtid in MariaDB.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.domain_id, self.server_id, self.sequence)
    }
}
