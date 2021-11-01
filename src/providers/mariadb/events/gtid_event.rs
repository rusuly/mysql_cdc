use crate::providers::mariadb::gtid::gtid::Gtid;
use crate::{errors::Error, events::event_header::EventHeader};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

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
    /// Parses events in MariaDB 10.0.2+.
    pub fn parse(cursor: &mut Cursor<&[u8]>, header: &EventHeader) -> Result<Self, Error> {
        let sequence = cursor.read_u64::<LittleEndian>()?;
        let domain_id = cursor.read_u32::<LittleEndian>()?;
        let flags = cursor.read_u8()?;

        let gtid = Gtid::new(domain_id, header.server_id, sequence);
        Ok(Self { gtid, flags })
    }
}
