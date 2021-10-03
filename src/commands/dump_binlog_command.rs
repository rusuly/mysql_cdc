use crate::commands::command_type::CommandType;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Cursor, Write};

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

    pub fn serialize(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        let mut cursor = Cursor::new(&mut vec);

        cursor.write_u8(CommandType::BINLOG_DUMP as u8).unwrap();
        cursor
            .write_u32::<LittleEndian>(self.binlog_position)
            .unwrap();
        cursor.write_u16::<LittleEndian>(self.flags).unwrap();
        cursor.write_u32::<LittleEndian>(self.server_id).unwrap();
        cursor.write(self.binlog_filename.as_bytes()).unwrap();

        vec
    }
}
