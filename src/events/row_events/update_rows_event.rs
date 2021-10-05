use crate::events::row_events::row_data::UpdateRowData;
use crate::events::row_events::row_parser::{parse_head, parse_update_row_data_list};
use crate::events::table_map_event::TableMapEvent;
use crate::extensions::read_bitmap_little_endian;
use std::collections::HashMap;
use std::io::Cursor;

/// Represents one or many updated rows in row based replication.
/// Includes versions before and after update.
/// <a href="https://mariadb.com/kb/en/library/rows_event_v1/">See more</a>
#[derive(Debug)]
pub struct UpdateRowsEvent {
    /// Gets id of the table where rows were updated
    pub table_id: u64,

    /// Gets <a href="https://mariadb.com/kb/en/rows_event_v1/#flags">flags</a>
    pub flags: u16,

    /// Gets number of columns in the table
    pub columns_number: usize,

    /// Gets bitmap of columns present in row event before update. See binlog_row_image parameter.
    pub columns_before_update: Vec<bool>,

    /// Gets bitmap of columns present in row event after update. See binlog_row_image parameter.
    pub columns_after_update: Vec<bool>,

    /// Gets updated rows
    pub rows: Vec<UpdateRowData>,
}

impl UpdateRowsEvent {
    /// Supports all versions of MariaDB and MySQL 5.5+ (V1 and V2 row events).
    pub fn parse(
        cursor: &mut Cursor<&[u8]>,
        table_map: &HashMap<u64, TableMapEvent>,
        row_event_version: u8,
    ) -> Self {
        let (table_id, flags, columns_number) = parse_head(cursor, row_event_version);
        let columns_before_update = read_bitmap_little_endian(cursor, columns_number);
        let columns_after_update = read_bitmap_little_endian(cursor, columns_number);
        let rows = parse_update_row_data_list(
            cursor,
            table_map,
            table_id,
            &columns_before_update,
            &columns_after_update,
        );
        Self {
            table_id,
            flags,
            columns_number,
            columns_before_update,
            columns_after_update,
            rows,
        }
    }
}
