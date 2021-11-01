use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Cursor};

/// EOF packet marks the end of a resultset and returns status and warnings.
/// <a href="https://mariadb.com/kb/en/library/eof_packet/">See more</a>
#[derive(Debug)]
pub struct EndOfFilePacket {
    pub warning_count: u16,
    pub server_status: u16,
}

impl EndOfFilePacket {
    pub fn parse(packet: &[u8]) -> Result<Self, io::Error> {
        let mut cursor = Cursor::new(packet);

        let warning_count = cursor.read_u16::<LittleEndian>()?;
        let server_status = cursor.read_u16::<LittleEndian>()?;

        Ok(Self {
            warning_count,
            server_status,
        })
    }
}
