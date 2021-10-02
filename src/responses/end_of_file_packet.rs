use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

/// EOF packet marks the end of a resultset and returns status and warnings.
/// <a href="https://mariadb.com/kb/en/library/eof_packet/">See more</a>
#[derive(Debug)]
pub struct EndOfFilePacket {
    pub warning_count: u16,
    pub server_status: u16,
}

impl EndOfFilePacket {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Self {
        let warning_count = cursor.read_u16::<LittleEndian>().unwrap();
        let server_status = cursor.read_u16::<LittleEndian>().unwrap();

        Self {
            warning_count,
            server_status,
        }
    }
}
