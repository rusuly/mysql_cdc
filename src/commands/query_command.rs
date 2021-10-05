use crate::commands::command_type::CommandType;
use byteorder::WriteBytesExt;
use std::io::{Cursor, Write};

/// COM_QUERY sends the server an SQL statement to be executed immediately.
/// <a href="https://mariadb.com/kb/en/library/com_query/">See more</a>
pub struct QueryCommand {
    pub sql: String,
}

impl QueryCommand {
    pub fn new(sql: String) -> Self {
        Self { sql }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        let mut cursor = Cursor::new(&mut vec);

        cursor.write_u8(CommandType::Query as u8).unwrap();
        cursor.write(self.sql.as_bytes()).unwrap();

        vec
    }
}
