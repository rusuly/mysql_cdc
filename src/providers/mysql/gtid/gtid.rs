use crate::providers::mysql::gtid::uuid::Uuid;
use std::fmt;

/// MySQL 5.6+ representation of Gtid.
#[derive(Debug)]
pub struct Gtid {
    /// Gets identifier of the original server that generated the event.
    pub source_id: Uuid,

    /// Gets sequence number of the event on the original server.
    pub transaction_id: u64,
}

impl Gtid {
    pub fn new(source_id: Uuid, transaction_id: u64) -> Self {
        Self {
            source_id,
            transaction_id,
        }
    }
}

impl fmt::Display for Gtid {
    /// Returns string representation of Gtid in MySQL Server.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.source_id, self.transaction_id)
    }
}
