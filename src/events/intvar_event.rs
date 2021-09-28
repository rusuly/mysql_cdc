use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

/// Generated when an auto increment column or LAST_INSERT_ID() function are used.
/// <a href="https://mariadb.com/kb/en/library/intvar_event/">See more</a>
#[derive(Debug)]
pub struct IntVarEvent {
    /// Gets type.
    /// 0x00 - Invalid value.
    /// 0x01 - LAST_INSERT_ID.
    /// 0x02 - Insert id (auto_increment).
    pub intvar_type: u8,

    /// Gets value.
    pub value: u64,
}

impl IntVarEvent {
    /// Supports all versions of MariaDB and MySQL.
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Self {
        let intvar_type = cursor.read_u8().unwrap();
        let value = cursor.read_u64::<LittleEndian>().unwrap();

        Self { intvar_type, value }
    }
}
