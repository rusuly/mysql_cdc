use crate::providers::mysql::gtid::interval::Interval;
use crate::providers::mysql::gtid::uuid::Uuid;
use crate::providers::mysql::gtid::uuid_set::UuidSet;
use crate::{errors::Error, providers::mysql::gtid::gtid_set::GtidSet};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

/// Used to record the gtid_executed of previous binlog files.
#[derive(Debug)]
pub struct PreviousGtidsEvent {
    /// Gets GtidSet of previous files.
    pub gtid_set: GtidSet,
}

impl PreviousGtidsEvent {
    /// Parses events in MySQL 5.6+.
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let uuid_set_number = cursor.read_u64::<LittleEndian>()?;
        let mut gtid_set = GtidSet::new();

        for _i in 0..uuid_set_number {
            let mut source_id = [0u8; 16];
            cursor.read_exact(&mut source_id)?;
            let source_id = Uuid::new(source_id);

            let mut uuid_set = UuidSet::new(source_id, Vec::new());
            let interval_number = cursor.read_u64::<LittleEndian>()?;
            for _y in 0..interval_number {
                let start = cursor.read_u64::<LittleEndian>()?;
                let end = cursor.read_u64::<LittleEndian>()?;
                uuid_set.intervals.push(Interval::new(start, end - 1));
            }
            gtid_set
                .uuid_sets
                .insert(uuid_set.source_id.uuid.clone(), uuid_set);
        }

        Ok(Self { gtid_set })
    }
}
