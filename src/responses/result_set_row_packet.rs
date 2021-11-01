use crate::{errors::Error, extensions::read_len_enc_str};
use std::io::Cursor;

/// Returned in response to a QueryCommand.
/// <a href="https://mariadb.com/kb/en/library/resultset/">See more</a>
#[derive(Debug)]
pub struct ResultSetRowPacket {
    pub cells: Vec<String>,
}

impl ResultSetRowPacket {
    pub fn parse(packet: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(packet);

        let len = cursor.get_ref().len() as u64;
        let mut cells = Vec::new();

        while cursor.position() < len {
            cells.push(read_len_enc_str(&mut cursor)?);
        }

        Ok(Self { cells })
    }
}
