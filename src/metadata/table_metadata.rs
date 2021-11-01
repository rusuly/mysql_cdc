use crate::constants::column_type::ColumnType;
use crate::errors::Error;
use crate::extensions::{read_len_enc_num, read_len_enc_str};
use crate::metadata::default_charset::DefaultCharset;
use crate::metadata::metadata_type::MetadataType;
use byteorder::ReadBytesExt;
use std::io::{self, Cursor, Read};

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
    pub fn parse(cursor: &mut Cursor<&[u8]>, column_types: &[u8]) -> Result<Self, Error> {
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
            let metadata_type = MetadataType::from_code(cursor.read_u8()?)?;
            let metadata_length = read_len_enc_num(cursor)?;

            let mut metadata = vec![0u8; metadata_length];
            cursor.read_exact(&mut metadata)?;

            let mut buffer = Cursor::new(metadata.as_slice());
            match metadata_type {
                MetadataType::Signedness => {
                    let count = get_numeric_column_count(column_types)?;
                    signedness = Some(read_bitmap_reverted(&mut buffer, count)?);
                }
                MetadataType::DefaultCharset => {
                    default_charset = Some(parse_default_charser(&mut buffer)?);
                }
                MetadataType::ColumnCharset => {
                    column_charsets = Some(parse_int_array(&mut buffer)?);
                }
                MetadataType::ColumnName => {
                    column_names = Some(parse_string_array(&mut buffer)?);
                }
                MetadataType::SetStrValue => {
                    set_string_values = Some(parse_type_values(&mut buffer)?);
                }
                MetadataType::EnumStrValue => {
                    enum_string_values = Some(parse_type_values(&mut buffer)?);
                }
                MetadataType::GeometryType => {
                    geometry_types = Some(parse_int_array(&mut buffer)?);
                }
                MetadataType::SimplePrimaryKey => {
                    simple_primary_keys = Some(parse_int_array(&mut buffer)?);
                }
                MetadataType::PrimaryKeyWithPrefix => {
                    primary_keys_with_prefix = Some(parse_int_map(&mut buffer)?);
                }
                MetadataType::EnumAndSetDefaultCharset => {
                    enum_and_set_default_charset = Some(parse_default_charser(&mut buffer)?);
                }
                MetadataType::EnumAndSetColumnCharset => {
                    enum_and_set_column_charsets = Some(parse_int_array(&mut buffer)?);
                }
                MetadataType::ColumnVisibility => {
                    column_visibility =
                        Some(read_bitmap_reverted(&mut buffer, column_types.len())?);
                }
            }
        }

        Ok(Self {
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
        })
    }
}

fn parse_int_array(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u32>, Error> {
    let mut result = Vec::new();
    while cursor.position() < cursor.get_ref().len() as u64 {
        let value = read_len_enc_num(cursor)?;
        result.push(value as u32);
    }
    Ok(result)
}

fn parse_string_array(cursor: &mut Cursor<&[u8]>) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();
    while cursor.position() < cursor.get_ref().len() as u64 {
        let value = read_len_enc_str(cursor)?;
        result.push(value);
    }
    Ok(result)
}

fn parse_int_map(cursor: &mut Cursor<&[u8]>) -> Result<Vec<(u32, u32)>, Error> {
    let mut result = Vec::new();
    while cursor.position() < cursor.get_ref().len() as u64 {
        let key = read_len_enc_num(cursor)?;
        let value = read_len_enc_num(cursor)?;
        result.push((key as u32, value as u32));
    }
    Ok(result)
}

fn parse_type_values(cursor: &mut Cursor<&[u8]>) -> Result<Vec<Vec<String>>, Error> {
    let mut result = Vec::new();
    while cursor.position() < cursor.get_ref().len() as u64 {
        let length = read_len_enc_num(cursor)?;
        let mut type_values = Vec::new();
        for _i in 0..length {
            type_values.push(read_len_enc_str(cursor)?);
        }
        result.push(type_values);
    }
    Ok(result)
}

fn parse_default_charser(cursor: &mut Cursor<&[u8]>) -> Result<DefaultCharset, Error> {
    let default_collation = read_len_enc_num(cursor)?;
    let charset_collations = parse_int_map(cursor)?;
    Ok(DefaultCharset::new(
        default_collation as u32,
        charset_collations,
    ))
}

fn read_bitmap_reverted(
    cursor: &mut Cursor<&[u8]>,
    bits_number: usize,
) -> Result<Vec<bool>, io::Error> {
    let mut result = vec![false; bits_number];
    let bytes_number = (bits_number + 7) / 8;
    for i in 0..bytes_number {
        let value = cursor.read_u8()?;
        for y in 0..8 {
            let index = (i << 3) + y;
            if index == bits_number {
                break;
            }

            // The difference from ReadBitmap is that bits are reverted
            result[index] = (value & (1 << (7 - y))) > 0;
        }
    }
    Ok(result)
}

fn get_numeric_column_count(column_types: &[u8]) -> Result<usize, Error> {
    let mut count = 0;
    for i in 0..column_types.len() {
        match ColumnType::from_code(column_types[i])? {
            ColumnType::Tiny => count += 1,
            ColumnType::Short => count += 1,
            ColumnType::Int24 => count += 1,
            ColumnType::Long => count += 1,
            ColumnType::LongLong => count += 1,
            ColumnType::Float => count += 1,
            ColumnType::Double => count += 1,
            ColumnType::NewDecimal => count += 1,
            _ => (),
        }
    }
    Ok(count)
}
