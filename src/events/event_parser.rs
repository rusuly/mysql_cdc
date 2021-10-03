use crate::constants::checksum_type::ChecksumType;
use crate::events::binlog_event::BinlogEvent;
use crate::events::event_header::EventHeader;
use crate::events::event_type::EventType;
use crate::events::format_description_event::FormatDescriptionEvent;
use crate::events::heartbeat_event::HeartbeatEvent;
use crate::events::intvar_event::IntVarEvent;
use crate::events::query_event::QueryEvent;
use crate::events::rotate_event::RotateEvent;
use crate::events::rows_query_event::RowsQueryEvent;
use crate::events::xid_event::XidEvent;
use std::io::Cursor;

pub struct EventParser {
    /// Gets checksum algorithm type used in a binlog file.
    pub checksum_type: ChecksumType,
}

impl EventParser {
    pub fn new() -> Self {
        Self {
            checksum_type: ChecksumType::NONE,
        }
    }

    pub fn parse_event(&mut self, header: &EventHeader, slice: &[u8]) -> BinlogEvent {
        // Consider verifying checksum
        let mut cursor = match self.checksum_type {
            ChecksumType::NONE => Cursor::new(slice),
            ChecksumType::CRC32 => Cursor::new(&slice[0..slice.len() - 4]),
        };

        let binlog_event: BinlogEvent = match EventType::from_code(header.event_type) {
            EventType::FORMAT_DESCRIPTION_EVENT => BinlogEvent::FormatDescriptionEvent(
                FormatDescriptionEvent::parse(&mut cursor, &header),
            ),
            /*EventType::TABLE_MAP_EVENT => TableMapEvent::parse(slice),*/
            EventType::HEARTBEAT_EVENT => {
                BinlogEvent::HeartbeatEvent(HeartbeatEvent::parse(&mut cursor))
            }
            EventType::ROTATE_EVENT => BinlogEvent::RotateEvent(RotateEvent::parse(&mut cursor)),
            //
            EventType::INTVAR_EVENT => BinlogEvent::IntVarEvent(IntVarEvent::parse(&mut cursor)),
            EventType::QUERY_EVENT => BinlogEvent::QueryEvent(QueryEvent::parse(&mut cursor)),
            EventType::XID_EVENT => BinlogEvent::XidEvent(XidEvent::parse(&mut cursor)),
            // Rows events used in MariaDB and MySQL from 5.1.15 to 5.6.
            /*EventType::WRITE_ROWS_EVENT_V1 => WriteRowsEvent::parse(slice, &self.table_map, 1),
            EventType::UPDATE_ROWS_EVENT_V1 => UpdateRowsEvent::parse(slice, &self.table_map, 1),
            EventType::DELETE_ROWS_EVENT_V1 => DeleteRowsEvent::parse(slice, &self.table_map, 1),
            // Rows events used only in MySQL from 5.6 to 8.0.
            EventType::MYSQL_WRITE_ROWS_EVENT_V2 => WriteRowsEvent::parse(slice, &self.table_map, 2),
            EventType::MYSQL_UPDATE_ROWS_EVENT_V2 => UpdateRowsEvent::parse(slice, &self.table_map, 2),
            EventType::MYSQL_DELETE_ROWS_EVENT_V2 => DeleteRowsEvent::parse(slice, &self.table_map, 2),
            // MySQL specific events
            EventType::MYSQL_GTID_EVENT => GtidEvent::parse_mysql(slice),*/
            EventType::MYSQL_ROWS_QUERY_EVENT => {
                BinlogEvent::RowsQueryEvent(RowsQueryEvent::parse_mysql(&mut cursor))
            }
            /*EventType::MYSQL_PREVIOUS_GTIDS_EVENT => PreviousGtidsEvent::parse(slice),
            EventType::MYSQL_XA_PREPARE => XaPrepareEvent::parse(slice),
            // MariaDB specific events
            EventType::MARIADB_GTID_EVENT => GtidEvent::parse_mariadb(slice),
            EventType::MARIADB_GTID_LIST_EVENT => GtidListEvent::parse(slice),*/
            EventType::MARIADB_ANNOTATE_ROWS_EVENT => {
                BinlogEvent::RowsQueryEvent(RowsQueryEvent::parse_mariadb(&mut cursor))
            }
            _ => BinlogEvent::UnknownEvent,
        };

        if let BinlogEvent::FormatDescriptionEvent(x) = &binlog_event {
            self.checksum_type = x.checksum_type;
        }

        binlog_event
    }
}
