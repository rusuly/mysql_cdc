use std::io::{Cursor, Read};

/// The event is sent from master to the client for keep alive feature.
/// <a href="https://mariadb.com/kb/en/library/heartbeat_log_event/">See more</a>
#[derive(Debug)]
pub struct HeartbeatEvent {
    /// Gets current master binlog filename
    pub binlog_filename: String,
}

impl HeartbeatEvent {
    /// Supports all versions of MariaDB and MySQL.
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Self {
        let mut binlog_filename = String::new();
        cursor.read_to_string(&mut binlog_filename).unwrap();

        Self { binlog_filename }
    }
}
