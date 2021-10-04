use crate::constants::column_type::ColumnType;
use crate::extensions::{read_len_enc_num, read_len_enc_str};
use crate::metadata::default_charset::DefaultCharset;
use crate::metadata::metadata_type::MetadataType;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

/// Contains metadata for table columns.
/// <a href="https://dev.mysql.com/doc/dev/mysql-server/latest/classbinary__log_1_1Table__map__event.html">See more</a>
#[derive(Clone, Debug)]
pub struct TableMetadata {
    /// Gets signedness of numeric colums.
    pub signedness: Option<Vec<bool>>,

    /// Gets charsets of character columns.
    pub default_charset: Option<DefaultCharset>,

    /// Gets charsets of character columns.
    pub column_charsets: Option<Vec<u32>>,

    /// Gets column names.
    pub column_names: Option<Vec<String>>,

    /// Gets string values of SET columns.
    pub set_string_values: Option<Vec<Vec<String>>>,

    /// Gets string values of ENUM columns
    pub enum_string_values: Option<Vec<Vec<String>>>,

    /// Gets real types of geometry columns.
    pub geometry_types: Option<Vec<u32>>,

    /// Gets primary keys without prefixes.
    pub simple_primary_keys: Option<Vec<u32>>,

    /// Gets primary keys with prefixes.
    pub primary_keys_with_prefix: Option<Vec<(u32, u32)>>,

    /// Gets charsets of ENUM and SET columns.
    pub enum_and_set_default_charset: Option<DefaultCharset>,

    /// Gets charsets of ENUM and SET columns.
    pub enum_and_set_column_charsets: Option<Vec<u32>>,

    /// Gets visibility attribute of columns.
    pub column_visibility: Option<Vec<bool>>,
}

impl TableMetadata {
    pub fn parse(cursor: &mut Cursor<&[u8]>, column_types: &[u8]) -> Self {
        let mut signedness = None;
        let mut default_charset = None;
        let mut column_charsets = None;
        let mut column_names = None;
        let mut set_string_values = None;
        let mut enum_string_values = None;
        let mut geometry_types = None;
        let mut simple_primary_keys = None;
        let mut primary_keys_with_prefix = None;
        let mut enum_and_set_default_charset = None;
        let mut enum_and_set_column_charsets = None;
        let mut column_visibility = None;

        while cursor.position() < cursor.get_ref().len() as u64 {
            let metadata_type = MetadataType::from_code(cursor.read_u8().unwrap());
            let metadata_length = read_len_enc_num(cursor);

            let mut metadata = vec![0u8; metadata_length];
            cursor.read_exact(&mut metadata).unwrap();

            let mut buffer = Cursor::new(metadata.as_slice());
            match metadata_type {
                MetadataType::SIGNEDNESS => {
                    let count = get_numeric_column_count(column_types);
                    signedness = Some(read_bitmap_reverted(&mut buffer, count));
                }
                MetadataType::DEFAULT_CHARSET => {
                    default_charset = Some(parse_default_charser(&mut buffer));
                }
                MetadataType::COLUMN_CHARSET => {
                    column_charsets = Some(parse_int_array(&mut buffer));
                }
                MetadataType::COLUMN_NAME => {
                    column_names = Some(parse_string_array(&mut buffer));
                }
                MetadataType::SET_STR_VALUE => {
                    set_string_values = Some(parse_type_values(&mut buffer));
                }
                MetadataType::ENUM_STR_VALUE => {
                    enum_string_values = Some(parse_type_values(&mut buffer));
                }
                MetadataType::GEOMETRY_TYPE => {
                    geometry_types = Some(parse_int_array(&mut buffer));
                }
                MetadataType::SIMPLE_PRIMARY_KEY => {
                    simple_primary_keys = Some(parse_int_array(&mut buffer));
                }
                MetadataType::PRIMARY_KEY_WITH_PREFIX => {
                    primary_keys_with_prefix = Some(parse_int_map(&mut buffer));
                }
                MetadataType::ENUM_AND_SET_DEFAULT_CHARSET => {
                    enum_and_set_default_charset = Some(parse_default_charser(&mut buffer));
                }
                MetadataType::ENUM_AND_SET_COLUMN_CHARSET => {
                    enum_and_set_column_charsets = Some(parse_int_array(&mut buffer));
                }
                MetadataType::COLUMN_VISIBILITY => {
                    column_visibility = Some(read_bitmap_reverted(&mut buffer, column_types.len()));
                }
            }
        }

        Self {
            signedness,
            default_charset,
            column_charsets,
            column_names,
            set_string_values,
            enum_string_values,
            geometry_types,
            simple_primary_keys,
            primary_keys_with_prefix,
            enum_and_set_default_charset,
            enum_and_set_column_charsets,
            column_visibility,
        }
    }
}

fn parse_int_array(cursor: &mut Cursor<&[u8]>) -> Vec<u32> {
    let mut result = Vec::new();
    while cursor.position() < cursor.get_ref().len() as u64 {
        let value = read_len_enc_num(cursor);
        result.push(value as u32);
    }
    result
}

fn parse_string_array(cursor: &mut Cursor<&[u8]>) -> Vec<String> {
    let mut result = Vec::new();
    while cursor.position() < cursor.get_ref().len() as u64 {
        let value = read_len_enc_str(cursor);
        result.push(value);
    }
    result
}

fn parse_int_map(cursor: &mut Cursor<&[u8]>) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    while cursor.position() < cursor.get_ref().len() as u64 {
        let key = read_len_enc_num(cursor);
        let value = read_len_enc_num(cursor);
        result.push((key as u32, value as u32));
    }
    result
}

fn parse_type_values(cursor: &mut Cursor<&[u8]>) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    while cursor.position() < cursor.get_ref().len() as u64 {
        let length = read_len_enc_num(cursor);
        let mut type_values = Vec::new();
        for i in 0..length {
            type_values.push(read_len_enc_str(cursor));
        }
        result.push(type_values);
    }
    result
}

fn parse_default_charser(cursor: &mut Cursor<&[u8]>) -> DefaultCharset {
    let default_collation = read_len_enc_num(cursor);
    let charset_collations = parse_int_map(cursor);
    return DefaultCharset::new(default_collation as u32, charset_collations);
}

fn read_bitmap_reverted(cursor: &mut Cursor<&[u8]>, bits_number: usize) -> Vec<bool> {
    let mut result = vec![false; bits_number];
    let bytes_number = (bits_number + 7) / 8;
    for i in 0..bytes_number {
        let value = cursor.read_u8().unwrap();
        for y in 0..8 {
            let index = (i << 3) + y;
            if index == bits_number {
                break;
            }

            // The difference from ReadBitmap is that bits are reverted
            result[index] = (value & (1 << (7 - y))) > 0;
        }
    }
    result
}

fn get_numeric_column_count(column_types: &[u8]) -> usize {
    let mut count = 0;
    for i in 0..column_types.len() {
        match ColumnType::from_code(column_types[i]) {
            ColumnType::TINY => count += 1,
            ColumnType::SHORT => count += 1,
            ColumnType::INT24 => count += 1,
            ColumnType::LONG => count += 1,
            ColumnType::LONGLONG => count += 1,
            ColumnType::FLOAT => count += 1,
            ColumnType::DOUBLE => count += 1,
            ColumnType::NEWDECIMAL => count += 1,
            _ => (),
        }
    }
    count
}
