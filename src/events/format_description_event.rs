use crate::constants;
use crate::constants::checksum_type::ChecksumType;
use crate::events::event_header::EventHeader;
use crate::events::event_type::EventType;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom};

const EVENT_TYPES_OFFSET: u8 = 2 + 50 + 4 + 1;

/// Written as the first event in binlog file or when replication is started.
/// See <a href="https://mariadb.com/kb/en/library/format_description_event/">MariaDB docs</a>
/// See <a href="https://dev.mysql.com/doc/internals/en/format-description-event.html">MySQL docs</a>
/// See <a href="https://mariadb.com/kb/en/library/5-slave-registration/#events-transmission-after-com_binlog_dump">start events flow</a>
#[derive(Debug)]
pub struct FormatDescriptionEvent {
    /// Gets binary log format version. This should always be 4.
    pub binlog_version: u16,

    /// Gets MariaDB/MySQL server version name.
    pub server_version: String,

    /// Gets checksum algorithm type.
    pub checksum_type: ChecksumType,
}

impl FormatDescriptionEvent {
    /// Supports all versions of MariaDB and MySQL 5.0+ (V4 header format).
    pub fn parse(cursor: &mut Cursor<&[u8]>, header: &EventHeader) -> Self {
        let binlog_version = cursor.read_u16::<LittleEndian>().unwrap();

        // Read server version
        let mut server_version = [0u8; 50];
        cursor.read_exact(&mut server_version).unwrap();
        let mut slice: &[u8] = &server_version;
        if let Some(zero_index) = server_version.iter().position(|&b| b == 0) {
            slice = &server_version[..zero_index];
        }
        let server_version = std::str::from_utf8(slice).unwrap().to_string();

        // Redundant timestamp & header length which is always 19
        cursor.seek(SeekFrom::Current(5)).unwrap();

        // Get size of the event payload to determine beginning of the checksum part
        cursor
            .seek(SeekFrom::Current(
                EventType::FORMAT_DESCRIPTION_EVENT as i64 - 1,
            ))
            .unwrap();
        let payload_length = cursor.read_u8().unwrap();

        let mut checksum_type = ChecksumType::NONE;
        if payload_length != header.event_length as u8 - constants::EVENT_HEADER_SIZE as u8 {
            let skip = payload_length as i64
                - EVENT_TYPES_OFFSET as i64
                - EventType::FORMAT_DESCRIPTION_EVENT as i64;

            cursor.seek(SeekFrom::Current(skip)).unwrap();
            checksum_type = ChecksumType::from_code(cursor.read_u8().unwrap());
        }

        Self {
            binlog_version,
            server_version,
            checksum_type,
        }
    }
}
