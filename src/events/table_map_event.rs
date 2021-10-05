use crate::constants::column_type::ColumnType;
use crate::extensions::{read_bitmap_little_endian, read_len_enc_num, read_string};
use crate::metadata::table_metadata::TableMetadata;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom};

/// The event has table defition for row events.
/// <a href="https://mariadb.com/kb/en/library/table_map_event/">See more</a>
#[derive(Clone, Debug)]
pub struct TableMapEvent {
    /// Gets id of the changed table
    pub table_id: u64,

    /// Gets database name of the changed table
    pub database_name: String,

    /// Gets name of the changed table
    pub table_name: String,

    /// Gets column types of the changed table
    pub column_types: Vec<u8>,

    /// Gets columns metadata
    pub column_metadata: Vec<u16>,

    /// Gets columns nullability
    pub null_bitmap: Vec<bool>,

    /// Gets table metadata for MySQL 5.6+
    pub table_metadata: Option<TableMetadata>,
}

impl TableMapEvent {
    /// Supports all versions of MariaDB and MySQL 5.0+.
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Self {
        let table_id = cursor.read_u48::<LittleEndian>().unwrap();

        // Reserved bytes
        cursor.seek(SeekFrom::Current(2)).unwrap();

        // Database name is null terminated
        let database_name_length = cursor.read_u8().unwrap();
        let database_name = read_string(cursor, database_name_length as usize);
        cursor.seek(SeekFrom::Current(1)).unwrap();

        // Table name is null terminated
        let table_name_length = cursor.read_u8().unwrap();
        let table_name = read_string(cursor, table_name_length as usize);
        cursor.seek(SeekFrom::Current(1)).unwrap();

        let columns_number = read_len_enc_num(cursor);
        let mut column_types = vec![0u8; columns_number];
        cursor.read_exact(&mut column_types).unwrap();

        let _metadata_length = read_len_enc_num(cursor);
        let column_metadata = TableMapEvent::parse_metadata(cursor, &column_types);

        let null_bitmap = read_bitmap_little_endian(cursor, columns_number);

        let mut table_metadata = None;
        if cursor.position() < cursor.get_ref().len() as u64 {
            // Table metadata is supported in MySQL 5.6+ and MariaDB 10.5+.
            table_metadata = Some(TableMetadata::parse(cursor, &column_types));
        }

        Self {
            table_id,
            database_name,
            table_name,
            column_types,
            column_metadata,
            null_bitmap,
            table_metadata,
        }
    }

    fn parse_metadata(cursor: &mut Cursor<&[u8]>, column_types: &Vec<u8>) -> Vec<u16> {
        let mut metadata = vec![0u16; column_types.len()];

        // See https://mariadb.com/kb/en/library/rows_event_v1/#column-data-formats
        for i in 0..column_types.len() {
            let column_type = ColumnType::from_code(column_types[i]);
            metadata[i] = match column_type {
                // 1 byte metadata
                ColumnType::Geometry => cursor.read_u8().unwrap() as u16,
                ColumnType::Json => cursor.read_u8().unwrap() as u16,
                ColumnType::TinyBlob => cursor.read_u8().unwrap() as u16,
                ColumnType::MediumBlob => cursor.read_u8().unwrap() as u16,
                ColumnType::LongBlob => cursor.read_u8().unwrap() as u16,
                ColumnType::Blob => cursor.read_u8().unwrap() as u16,
                ColumnType::Float => cursor.read_u8().unwrap() as u16,
                ColumnType::Double => cursor.read_u8().unwrap() as u16,
                ColumnType::TimeStamp2 => cursor.read_u8().unwrap() as u16,
                ColumnType::DateTime2 => cursor.read_u8().unwrap() as u16,
                ColumnType::Time2 => cursor.read_u8().unwrap() as u16,
                // 2 bytes little endian
                ColumnType::Bit => cursor.read_u16::<LittleEndian>().unwrap(),
                ColumnType::VarChar => cursor.read_u16::<LittleEndian>().unwrap(),
                ColumnType::VarString => cursor.read_u16::<LittleEndian>().unwrap(),
                ColumnType::NewDecimal => cursor.read_u16::<LittleEndian>().unwrap(),
                // 2 bytes big endian
                ColumnType::Enum => cursor.read_u16::<BigEndian>().unwrap(),
                ColumnType::Set => cursor.read_u16::<BigEndian>().unwrap(),
                ColumnType::String => cursor.read_u16::<BigEndian>().unwrap(),
                _ => 0,
            }
        }
        metadata
    }
}
