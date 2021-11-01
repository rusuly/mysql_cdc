use crate::constants::checksum_type::ChecksumType;
use crate::errors::Error;
use crate::events::binlog_event::BinlogEvent;
use crate::events::event_header::EventHeader;
use crate::events::event_type::EventType;
use crate::events::format_description_event::FormatDescriptionEvent;
use crate::events::heartbeat_event::HeartbeatEvent;
use crate::events::intvar_event::IntVarEvent;
use crate::events::query_event::QueryEvent;
use crate::events::rotate_event::RotateEvent;
use crate::events::row_events::delete_rows_event::DeleteRowsEvent;
use crate::events::row_events::update_rows_event::UpdateRowsEvent;
use crate::events::row_events::write_rows_event::WriteRowsEvent;
use crate::events::rows_query_event::RowsQueryEvent;
use crate::events::table_map_event::TableMapEvent;
use crate::events::xid_event::XidEvent;
use crate::providers::mariadb::events::gtid_event::GtidEvent as MariaDbGtidEvent;
use crate::providers::mariadb::events::gtid_list_event::GtidListEvent;
use crate::providers::mysql::events::gtid_event::GtidEvent as MySqlGtidEvent;
use crate::providers::mysql::events::prev_gtids_event::PreviousGtidsEvent;
use std::collections::HashMap;
use std::io::Cursor;

pub struct EventParser {
    /// Gets checksum algorithm type used in a binlog file.
    pub checksum_type: ChecksumType,

    /// Gets TableMapEvent cache required in row events.
    table_map: HashMap<u64, TableMapEvent>,
}

impl EventParser {
    pub fn new() -> Self {
        Self {
            checksum_type: ChecksumType::None,
            table_map: HashMap::new(),
        }
    }

    pub fn parse_event(
        &mut self,
        header: &EventHeader,
        slice: &[u8],
    ) -> Result<BinlogEvent, Error> {
        // Consider verifying checksum
        let mut cursor = match self.checksum_type {
            ChecksumType::None => Cursor::new(slice),
            ChecksumType::Crc32 => Cursor::new(&slice[0..slice.len() - 4]),
        };

        let binlog_event: BinlogEvent = match EventType::from_code(header.event_type) {
            EventType::FormatDescriptionEvent => BinlogEvent::FormatDescriptionEvent(
                FormatDescriptionEvent::parse(&mut cursor, &header)?,
            ),
            EventType::TableMapEvent => {
                BinlogEvent::TableMapEvent(TableMapEvent::parse(&mut cursor)?)
            }
            EventType::HeartbeatEvent => {
                BinlogEvent::HeartbeatEvent(HeartbeatEvent::parse(&mut cursor)?)
            }
            EventType::RotateEvent => BinlogEvent::RotateEvent(RotateEvent::parse(&mut cursor)?),
            EventType::IntvarEvent => BinlogEvent::IntVarEvent(IntVarEvent::parse(&mut cursor)?),
            EventType::QueryEvent => BinlogEvent::QueryEvent(QueryEvent::parse(&mut cursor)?),
            EventType::XidEvent => BinlogEvent::XidEvent(XidEvent::parse(&mut cursor)?),
            // Rows events used in MariaDB and MySQL from 5.1.15 to 5.6.
            EventType::WriteRowsEventV1 => {
                BinlogEvent::WriteRowsEvent(WriteRowsEvent::parse(&mut cursor, &self.table_map, 1)?)
            }
            EventType::UpdateRowsEventV1 => BinlogEvent::UpdateRowsEvent(UpdateRowsEvent::parse(
                &mut cursor,
                &self.table_map,
                1,
            )?),
            EventType::DeleteRowsEventV1 => BinlogEvent::DeleteRowsEvent(DeleteRowsEvent::parse(
                &mut cursor,
                &self.table_map,
                1,
            )?),
            // MySQL specific events. Rows events used only in MySQL from 5.6 to 8.0.
            EventType::MySqlWriteRowsEventV2 => {
                BinlogEvent::WriteRowsEvent(WriteRowsEvent::parse(&mut cursor, &self.table_map, 2)?)
            }
            EventType::MySqlUpdateRowsEventV2 => BinlogEvent::UpdateRowsEvent(
                UpdateRowsEvent::parse(&mut cursor, &self.table_map, 2)?,
            ),
            EventType::MySqlDeleteRowsEventV2 => BinlogEvent::DeleteRowsEvent(
                DeleteRowsEvent::parse(&mut cursor, &self.table_map, 2)?,
            ),
            EventType::MySqlRowsQueryEvent => {
                BinlogEvent::RowsQueryEvent(RowsQueryEvent::parse_mysql(&mut cursor)?)
            }
            EventType::MySqlGtidEvent => {
                BinlogEvent::MySqlGtidEvent(MySqlGtidEvent::parse(&mut cursor)?)
            }
            EventType::MySqlPreviousGtidsEvent => {
                BinlogEvent::MySqlPrevGtidsEvent(PreviousGtidsEvent::parse(&mut cursor)?)
            }
            // MariaDB specific events
            EventType::MariaDbGtidEvent => {
                BinlogEvent::MariaDbGtidEvent(MariaDbGtidEvent::parse(&mut cursor, &header)?)
            }
            EventType::MariaDbGtidListEvent => {
                BinlogEvent::MariaDbGtidListEvent(GtidListEvent::parse(&mut cursor)?)
            }
            EventType::MariaDbAnnotateRowsEvent => {
                BinlogEvent::RowsQueryEvent(RowsQueryEvent::parse_mariadb(&mut cursor)?)
            }
            _ => BinlogEvent::UnknownEvent,
        };

        if let BinlogEvent::FormatDescriptionEvent(x) = &binlog_event {
            self.checksum_type = x.checksum_type;
        }

        if let BinlogEvent::TableMapEvent(x) = &binlog_event {
            self.table_map.insert(x.table_id, x.clone()); //todo: optimize
        }

        Ok(binlog_event)
    }
}
