use crate::constants::FIRST_EVENT_POSITION;
use crate::providers::mariadb::gtid::gtid_list::GtidList;
use crate::providers::mysql::gtid::gtid_set::GtidSet;
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

    /// MySQL Global Transaction ID position to start replication from.
    /// See <a href="https://dev.mysql.com/doc/refman/8.0/en/replication-gtids-concepts.html">MySQL GTID</a>
    pub gtid_set: Option<GtidSet>,

    /// MariaDB Global Transaction ID position to start replication from.
    /// See <a href="https://mariadb.com/kb/en/library/gtid/">MariaDB GTID</a>
    pub gtid_list: Option<GtidList>,

    /// Gets replication starting strategy.
    pub starting_strategy: StartingStrategy,
}

impl BinlogOptions {
    /// Starts replication from first available binlog on master server.
    pub fn from_start() -> BinlogOptions {
        BinlogOptions {
            filename: String::new(),
            position: FIRST_EVENT_POSITION as u32,
            gtid_set: None,
            gtid_list: None,
            starting_strategy: StartingStrategy::FromStart,
        }
    }

    /// Starts replication from last master binlog position
    /// which will be read by BinlogClient on first connect.
    pub fn from_end() -> BinlogOptions {
        BinlogOptions {
            filename: String::new(),
            position: 0,
            gtid_set: None,
            gtid_list: None,
            starting_strategy: StartingStrategy::FromEnd,
        }
    }

    /// Starts replication from specified binlog filename and position.
    pub fn from_position(filename: String, position: u32) -> BinlogOptions {
        BinlogOptions {
            filename,
            position,
            gtid_set: None,
            gtid_list: None,
            starting_strategy: StartingStrategy::FromPosition,
        }
    }

    /// Starts replication from specified Global Transaction ID.
    pub fn from_mysql_gtid(gtid_set: GtidSet) -> BinlogOptions {
        BinlogOptions {
            filename: String::new(),
            position: FIRST_EVENT_POSITION as u32,
            gtid_set: Some(gtid_set),
            gtid_list: None,
            starting_strategy: StartingStrategy::FromGtid,
        }
    }

    pub fn from_mariadb_gtid(gtid_list: GtidList) -> BinlogOptions {
        BinlogOptions {
            filename: String::new(),
            position: FIRST_EVENT_POSITION as u32,
            gtid_set: None,
            gtid_list: Some(gtid_list),
            starting_strategy: StartingStrategy::FromGtid,
        }
    }
}
