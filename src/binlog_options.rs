use crate::constants::FIRST_EVENT_POSITION;
use crate::starting_strategy::StartingStrategy;

/// Replication options used when client connects to the server.
#[derive(Debug)]
pub struct BinlogOptions {
    /// Binary log file name.
    /// The value is automatically changed on the RotateEvent.
    /// On reconnect the client resumes replication from the current position.
    pub filename: String,

    /// Binary log file position.
    /// The value is automatically changed when an event is successfully processed by a client.
    /// On reconnect the client resumes replication from the current position.
    pub position: u32,

    /// Gets replication starting strategy.
    pub starting_strategy: StartingStrategy,
}

impl BinlogOptions {
    /// Starts replication from first available binlog on master server.
    pub fn from_start() -> BinlogOptions {
        BinlogOptions {
            filename: String::new(),
            position: FIRST_EVENT_POSITION as u32,
            starting_strategy: StartingStrategy::FromStart,
        }
    }

    /// Starts replication from last master binlog position
    /// which will be read by BinlogClient on first connect.
    pub fn from_end() -> BinlogOptions {
        BinlogOptions {
            filename: String::new(),
            position: 0,
            starting_strategy: StartingStrategy::FromEnd,
        }
    }

    /// Starts replication from specified binlog filename and position.
    pub fn from_position(filename: String, position: u32) -> BinlogOptions {
        BinlogOptions {
            filename,
            position,
            starting_strategy: StartingStrategy::FromPosition,
        }
    }
}
