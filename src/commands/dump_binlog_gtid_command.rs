use crate::commands::command_type::CommandType;
use crate::providers::mysql::gtid::gtid_set::GtidSet;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{self, Cursor, Write};

/// Requests binlog event stream by GtidSet.
/// <a href="https://dev.mysql.com/doc/internals/en/com-binlog-dump-gtid.html">See more</a>
pub struct DumpBinlogGtidCommand {
    pub server_id: u32,
    pub binlog_filename: String,
    pub binlog_position: u32,
    pub flags: u16,
}

impl DumpBinlogGtidCommand {
    pub fn new(server_id: u32, binlog_filename: String, binlog_position: u32) -> Self {
        Self {
            server_id,
            binlog_filename,
            binlog_position,
            flags: 0,
        }
    }

    pub fn serialize(&self, gtid_set: &GtidSet) -> Result<Vec<u8>, io::Error> {
        let mut vec = Vec::new();
        let mut cursor = Cursor::new(&mut vec);

        cursor.write_u8(CommandType::BinlogDumpGtid as u8)?;
        cursor.write_u16::<LittleEndian>(self.flags)?;
        cursor.write_u32::<LittleEndian>(self.server_id)?;

        let filename_len = self.binlog_filename.len() as u32;
        cursor.write_u32::<LittleEndian>(filename_len)?;
        cursor.write(self.binlog_filename.as_bytes())?;

        let position = self.binlog_position as u64;
        cursor.write_u64::<LittleEndian>(position)?;

        let mut data_length = 8; /* Number of uuid_sets */
        for uuid_set in gtid_set.uuid_sets.values() {
            data_length += 16; /* SourceId */
            data_length += 8; /* Number of intervals */
            data_length += uuid_set.intervals.len() * (8 + 8) /* Start-End */;
        }

        cursor.write_u32::<LittleEndian>(data_length as u32)?;
        cursor.write_u64::<LittleEndian>(gtid_set.uuid_sets.len() as u64)?;

        for uuid_set in gtid_set.uuid_sets.values() {
            cursor.write(&uuid_set.source_id.data)?;
            cursor.write_u64::<LittleEndian>(uuid_set.intervals.len() as u64)?;

            for interval in &uuid_set.intervals {
                cursor.write_u64::<LittleEndian>(interval.start)?;
                cursor.write_u64::<LittleEndian>(interval.end + 1)?;
            }
        }

        Ok(vec)
    }
}
