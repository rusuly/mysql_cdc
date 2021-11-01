use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

use crate::errors::Error;

/// Last event in a binlog file which points to next binlog file.
/// Fake version is also returned when replication is started.
/// <a href="https://mariadb.com/kb/en/library/rotate_event/">See more</a>
#[derive(Debug)]
pub struct RotateEvent {
    /// Gets next binlog filename
    pub binlog_filename: String,

    /// Gets next binlog position
    pub binlog_position: u64,
}

impl RotateEvent {
    /// Supports all versions of MariaDB and MySQL.
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let binlog_position = cursor.read_u64::<LittleEndian>()?;

        let mut binlog_filename = String::new();
        cursor.read_to_string(&mut binlog_filename)?;

        Ok(Self {
            binlog_position,
            binlog_filename,
        })
    }
}
