use crate::providers::mysql::gtid::uuid::Uuid;
use crate::{errors::Error, providers::mysql::gtid::gtid::Gtid};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

/// Marks start of a new event group(transaction).
/// <a href="https://mariadb.com/kb/en/gtid_event/">See more</a>
#[derive(Debug)]
pub struct GtidEvent {
    /// Gets Global Transaction ID of the event group.
    pub gtid: Gtid,

    /// Gets flags.
    pub flags: u8,
}

impl GtidEvent {
    /// Parses events in MySQL 5.6+.
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let flags = cursor.read_u8()?;

        let mut source_id = [0u8; 16];
        cursor.read_exact(&mut source_id)?;
        let source_id = Uuid::new(source_id);

        let transaction_id = cursor.read_u64::<LittleEndian>()?;

        let gtid = Gtid::new(source_id, transaction_id);
        Ok(Self { gtid, flags })
    }
}
