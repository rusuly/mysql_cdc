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

#[cfg(test)]
mod tests {
    use crate::providers::mysql::events::prev_gtids_event::PreviousGtidsEvent;
    use std::io::Cursor;

    #[test]
    fn parse_event_returns_gtid_set() {
        let payload: Vec<u8> = vec![
            2, 0, 0, 0, 0, 0, 0, 0, 181, 205, 22, 36, 95, 48, 17, 228, 180, 233, 16, 81, 114, 27,
            210, 65, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 241, 15, 108, 0, 0, 0, 0, 0,
            187, 66, 29, 38, 95, 48, 17, 228, 180, 233, 216, 157, 103, 43, 46, 248, 1, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 209, 97, 119, 0, 0, 0, 0, 0,
        ];
        let mut cursor = Cursor::new(payload.as_slice());

        let expected = String::from("b5cd1624-5f30-11e4-b4e9-1051721bd241:1-7081968,bb421d26-5f30-11e4-b4e9-d89d672b2ef8:1-7823824");
        let event = PreviousGtidsEvent::parse(&mut cursor).unwrap();
        assert_eq!(expected, event.gtid_set.to_string());
    }
}
