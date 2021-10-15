use crate::providers::mysql::gtid::gtid::Gtid;
use crate::providers::mysql::gtid::uuid::Uuid;
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
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Self {
        let flags = cursor.read_u8().unwrap();

        let mut source_id = [0u8; 16];
        cursor.read_exact(&mut source_id).unwrap();
        let source_id = Uuid::new(source_id);

        let transaction_id = cursor.read_u64::<LittleEndian>().unwrap();

        let gtid = Gtid::new(source_id, transaction_id);
        Self { gtid, flags }
    }
}
