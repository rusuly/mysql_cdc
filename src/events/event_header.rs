use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

/// Binlog event header version 4. Header size is 19 bytes.
/// See <a href="https://mariadb.com/kb/en/library/2-binlog-event-header/">MariaDB docs</a>
/// See <a href="https://dev.mysql.com/doc/internals/en/binlog-version.html">MySQL docs</a>
#[derive(Debug)]
pub struct EventHeader {
    /// Provides creation time in seconds from Unix.
    pub timestamp: u32,

    /// Gets type of the binlog event.
    pub event_type: u8,

    /// Gets id of the server that created the event.
    pub server_id: u32,

    /// Gets event length (header + event + checksum).
    pub event_length: u32,

    /// Gets file position of next event.
    pub next_event_position: u32,

    /// Gets event flags.
    /// See <a href="https://mariadb.com/kb/en/2-binlog-event-header/#event-flag">documentation</a>.
    pub event_flags: u16,
}

impl EventHeader {
    pub fn parse(slice: &[u8]) -> Self {
        let mut cursor = Cursor::new(slice);
        Self {
            timestamp: cursor.read_u32::<LittleEndian>().unwrap(),
            event_type: cursor.read_u8().unwrap(),
            server_id: cursor.read_u32::<LittleEndian>().unwrap(),
            event_length: cursor.read_u32::<LittleEndian>().unwrap(),
            next_event_position: cursor.read_u32::<LittleEndian>().unwrap(),
            event_flags: cursor.read_u16::<LittleEndian>().unwrap(),
        }
    }
}
