use crate::commands::command_type::CommandType;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{self, Cursor, Write};

/// Requests binlog event stream.
/// <a href="https://mariadb.com/kb/en/library/com_binlog_dump/">See more</a>
pub struct DumpBinlogCommand {
    pub server_id: u32,
    pub binlog_filename: String,
    pub binlog_position: u32,
    pub flags: u16,
}

impl DumpBinlogCommand {
    pub fn new(server_id: u32, binlog_filename: String, binlog_position: u32) -> Self {
        Self {
            server_id,
            binlog_filename,
            binlog_position,
            flags: 0,
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, io::Error> {
        let mut vec = Vec::new();
        let mut cursor = Cursor::new(&mut vec);

        cursor.write_u8(CommandType::BinlogDump as u8)?;
        cursor.write_u32::<LittleEndian>(self.binlog_position)?;
        cursor.write_u16::<LittleEndian>(self.flags)?;
        cursor.write_u32::<LittleEndian>(self.server_id)?;
        cursor.write(self.binlog_filename.as_bytes())?;

        Ok(vec)
    }
}
