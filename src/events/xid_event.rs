use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

use crate::errors::Error;

/// Represents a transaction commit event.
/// <a href="https://mariadb.com/kb/en/library/xid_event/">See more</a>
#[derive(Debug)]
pub struct XidEvent {
    /// Gets the XID transaction number
    pub xid: u64,
}

impl XidEvent {
    /// Supports all versions of MariaDB and MySQL.
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let xid = cursor.read_u64::<LittleEndian>()?;

        Ok(Self { xid })
    }
}
