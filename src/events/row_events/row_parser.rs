use crate::constants::column_type::ColumnType;
use crate::errors::Error;
use crate::events::row_events::col_parser::{
    parse_bit, parse_blob, parse_date, parse_date_time, parse_date_time2, parse_string, parse_time,
    parse_time2, parse_timestamp, parse_timestamp2, parse_year,
};
use crate::events::row_events::mysql_value::MySqlValue;
use crate::events::row_events::row_data::{RowData, UpdateRowData};
use crate::events::table_map_event::TableMapEvent;
use crate::extensions::{read_bitmap_little_endian, read_len_enc_num};
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io::{Cursor, Seek, SeekFrom};

use super::actual_string_type::get_actual_string_type;
use super::decimal::parse_decimal;

pub const TABLE_MAP_NOT_FOUND: &str =
    "No preceding TableMapEvent event was found for the row event. \
You possibly started replication in the middle of logical event group.";

/// Parsing row based events.
/// See <a href="https://mariadb.com/kb/en/library/rows_event_v1/">MariaDB rows version 1</a>
/// See <a href="https://dev.mysql.com/doc/internals/en/rows-event.html#write-rows-eventv2">MySQL rows version 1/2</a>
/// See <a href="https://github.com/shyiko/mysql-binlog-connector-java">AbstractRowsEventDataDeserializer</a>

pub fn parse_row_data_list(
    cursor: &mut Cursor<&[u8]>,
    table_map: &HashMap<u64, TableMapEvent>,
    table_id: u64,
    columns_present: &Vec<bool>,
) -> Result<Vec<RowData>, Error> {
    let table = match table_map.get(&table_id) {
        Some(x) => x,
        None => return Err(Error::String(TABLE_MAP_NOT_FOUND.to_string())),
    };

    let cells_included = get_bits_number(columns_present);
    let mut rows = Vec::new();
    while cursor.position() < cursor.get_ref().len() as u64 {
        rows.push(parse_row(cursor, table, columns_present, cells_included)?);
    }
    Ok(rows)
}

pub fn parse_update_row_data_list(
    cursor: &mut Cursor<&[u8]>,
    table_map: &HashMap<u64, TableMapEvent>,
    table_id: u64,
    columns_before_update: &Vec<bool>,
    columns_after_update: &Vec<bool>,
) -> Result<Vec<UpdateRowData>, Error> {
    let table = match table_map.get(&table_id) {
        Some(x) => x,
        None => return Err(Error::String(TABLE_MAP_NOT_FOUND.to_string())),
    };

    let cells_included_before_update = get_bits_number(columns_before_update);
    let cells_included_after_update = get_bits_number(columns_after_update);
    let mut rows = Vec::new();
    while cursor.position() < cursor.get_ref().len() as u64 {
        let row_before_update = parse_row(
            cursor,
            table,
            columns_before_update,
            cells_included_before_update,
        )?;
        let row_after_update = parse_row(
            cursor,
            table,
            columns_after_update,
            cells_included_after_update,
        )?;
        rows.push(UpdateRowData::new(row_before_update, row_after_update));
    }
    Ok(rows)
}

pub fn parse_head(
    cursor: &mut Cursor<&[u8]>,
    row_event_version: u8,
) -> Result<(u64, u16, usize), Error> {
    let table_id = cursor.read_u48::<LittleEndian>()?;
    let flags = cursor.read_u16::<LittleEndian>()?;

    // Ignore extra data from newer versions of events
    if row_event_version == 2 {
        let extra_data_length = cursor.read_u16::<LittleEndian>()?;
        let skip = extra_data_length as i64 - 2;
        cursor.seek(SeekFrom::Current(skip))?;
    }

    let columns_number = read_len_enc_num(cursor)?;
    Ok((table_id, flags, columns_number))
}

pub fn parse_row(
    cursor: &mut Cursor<&[u8]>,
    table_map: &TableMapEvent,
    columns_present: &Vec<bool>,
    cells_included: usize,
) -> Result<RowData, Error> {
    let mut row = Vec::with_capacity(table_map.column_types.len());
    let null_bitmap = read_bitmap_little_endian(cursor, cells_included)?;

    let mut skipped_columns = 0;
    for i in 0..table_map.column_types.len() {
        // Data is missing if binlog_row_image != full
        if !columns_present[i] {
            skipped_columns += 1;
            row.push(None);
        }
        // Column is present and has null value
        else if null_bitmap[i - skipped_columns] {
            row.push(None);
        }
        // Column has data
        else {
            let mut column_type = table_map.column_types[i];
            let mut metadata = table_map.column_metadata[i];
            if ColumnType::from_code(column_type)? == ColumnType::String {
                get_actual_string_type(&mut column_type, &mut metadata);
            }
            row.push(Some(parse_cell(cursor, column_type, metadata)?));
        }
    }
    Ok(RowData::new(row))
}

fn parse_cell(
    cursor: &mut Cursor<&[u8]>,
    column_type: u8,
    metadata: u16,
) -> Result<MySqlValue, Error> {
    let value = match ColumnType::from_code(column_type)? {
        /* Numeric types. The only place where numbers can be negative */
        ColumnType::Tiny => MySqlValue::TinyInt(cursor.read_u8()?),
        ColumnType::Short => MySqlValue::SmallInt(cursor.read_u16::<LittleEndian>()?),
        ColumnType::Int24 => MySqlValue::MediumInt(cursor.read_u24::<LittleEndian>()?),
        ColumnType::Long => MySqlValue::Int(cursor.read_u32::<LittleEndian>()?),
        ColumnType::LongLong => MySqlValue::BigInt(cursor.read_u64::<LittleEndian>()?),
        ColumnType::Float => MySqlValue::Float(cursor.read_f32::<LittleEndian>()?),
        ColumnType::Double => MySqlValue::Double(cursor.read_f64::<LittleEndian>()?),
        ColumnType::NewDecimal => MySqlValue::Decimal(parse_decimal(cursor, metadata)?),
        /* String types, includes varchar, varbinary & fixed char, binary */
        ColumnType::String => MySqlValue::String(parse_string(cursor, metadata)?),
        ColumnType::VarChar => MySqlValue::String(parse_string(cursor, metadata)?),
        ColumnType::VarString => MySqlValue::String(parse_string(cursor, metadata)?),
        /* BIT, ENUM, SET types */
        ColumnType::Bit => MySqlValue::Bit(parse_bit(cursor, metadata)?),
        ColumnType::Enum => {
            MySqlValue::Enum(cursor.read_uint::<LittleEndian>(metadata as usize)? as u32)
        }
        ColumnType::Set => {
            MySqlValue::Set(cursor.read_uint::<LittleEndian>(metadata as usize)? as u64)
        }
        /* Blob types. MariaDB always creates BLOB for first three */
        ColumnType::TinyBlob => MySqlValue::Blob(parse_blob(cursor, metadata)?),
        ColumnType::MediumBlob => MySqlValue::Blob(parse_blob(cursor, metadata)?),
        ColumnType::LongBlob => MySqlValue::Blob(parse_blob(cursor, metadata)?),
        ColumnType::Blob => MySqlValue::Blob(parse_blob(cursor, metadata)?),
        /* Date and time types */
        ColumnType::Year => MySqlValue::Year(parse_year(cursor, metadata)?),
        ColumnType::Date => MySqlValue::Date(parse_date(cursor, metadata)?),
        // Older versions of MySQL.
        ColumnType::Time => MySqlValue::Time(parse_time(cursor, metadata)?),
        ColumnType::TimeStamp => MySqlValue::Timestamp(parse_timestamp(cursor, metadata)?),
        ColumnType::DateTime => MySqlValue::DateTime(parse_date_time(cursor, metadata)?),
        // MySQL 5.6.4+ types. Supported from MariaDB 10.1.2.
        ColumnType::Time2 => MySqlValue::Time(parse_time2(cursor, metadata)?),
        ColumnType::TimeStamp2 => MySqlValue::Timestamp(parse_timestamp2(cursor, metadata)?),
        ColumnType::DateTime2 => MySqlValue::DateTime(parse_date_time2(cursor, metadata)?),
        /* MySQL-specific data types */
        ColumnType::Geometry => MySqlValue::Blob(parse_blob(cursor, metadata)?),
        ColumnType::Json => MySqlValue::Blob(parse_blob(cursor, metadata)?),
        _ => {
            return Err(Error::String(format!(
                "Parsing column type {:?} is not supported",
                ColumnType::from_code(column_type)?
            )))
        }
    };
    Ok(value)
}

/// Gets number of bits set in a bitmap.
fn get_bits_number(bitmap: &Vec<bool>) -> usize {
    bitmap.iter().filter(|&x| *x == true).count()
}
