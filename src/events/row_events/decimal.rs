use crate::errors::Error;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

/// See <a href="https://dev.mysql.com/doc/internals/en/date-and-time-data-type-representation.html">Docs</a>

const DIGITS_PER_INT: u8 = 9;
const COMPRESSED_BYTES: [u8; 10] = [0, 1, 1, 2, 2, 3, 3, 4, 4, 4];

pub fn parse_decimal(cursor: &mut Cursor<&[u8]>, metadata: u16) -> Result<String, Error> {
    let precision = metadata & 0xFF;
    let scale = (metadata >> 8) as u8;
    let integral = (precision - scale as u16) as u8;

    let uncompressed_integral = integral / DIGITS_PER_INT;
    let uncompressed_fractional = scale / DIGITS_PER_INT;
    let compressed_integral = integral - (uncompressed_integral * DIGITS_PER_INT);
    let compressed_fractional = scale - (uncompressed_fractional * DIGITS_PER_INT);

    let length = (uncompressed_integral << 2)
        + COMPRESSED_BYTES[compressed_integral as usize]
        + (uncompressed_fractional << 2)
        + COMPRESSED_BYTES[compressed_fractional as usize];

    // Format
    // [1-3 bytes]  [4 bytes]      [4 bytes]        [4 bytes]      [4 bytes]      [1-3 bytes]
    // [Compressed] [Uncompressed] [Uncompressed] . [Uncompressed] [Uncompressed] [Compressed]
    let mut value = vec![0; length as usize];
    cursor.read_exact(&mut value)?;
    let mut result = String::new();

    let negative = (value[0] & 0x80) == 0;
    value[0] ^= 0x80;

    if negative {
        result += "-";
        for i in 0..value.len() {
            value[i] ^= 0xFF;
        }
    }

    let mut buffer = Cursor::new(value.as_slice());

    let mut started = false;
    let mut size = COMPRESSED_BYTES[compressed_integral as usize];

    if size > 0 {
        let number = buffer.read_uint::<BigEndian>(size as usize)? as u32;
        if number > 0 {
            started = true;
            result += &number.to_string();
        }
    }
    for _i in 0..uncompressed_integral {
        let number = buffer.read_u32::<BigEndian>()?;
        if started {
            result += &format!("{val:0prec$}", prec = 9, val = number)
        } else if number > 0 {
            started = true;
            result += &number.to_string();
        }
    }

    // There has to be at least 0
    if !started {
        result += "0";
    }
    if scale > 0 {
        result += ".";
    }

    size = COMPRESSED_BYTES[compressed_fractional as usize];
    for _i in 0..uncompressed_fractional {
        let value = buffer.read_u32::<BigEndian>()?;
        result += &format!("{val:0prec$}", prec = 9, val = value)
    }
    if size > 0 {
        let value = buffer.read_uint::<BigEndian>(size as usize)? as u32;
        let precision = compressed_fractional as usize;
        result += &format!("{val:0prec$}", prec = precision, val = value)
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::events::row_events::decimal::parse_decimal;
    use byteorder::{LittleEndian, ReadBytesExt};
    use std::io::Cursor;

    #[test]
    fn parse_positive_number() {
        // decimal(65,10), column = '1234567890112233445566778899001112223334445556667778889.9900011112'
        let payload: Vec<u8> = vec![
            65, 10, 129, 13, 251, 56, 210, 6, 176, 139, 229, 33, 200, 92, 19, 0, 16, 248, 159, 19,
            239, 59, 244, 39, 205, 127, 73, 59, 2, 55, 215, 2,
        ];
        let mut cursor = Cursor::new(payload.as_slice());
        let metadata = cursor.read_u16::<LittleEndian>().unwrap();

        let expected =
            String::from("1234567890112233445566778899001112223334445556667778889.9900011112");
        assert_eq!(expected, parse_decimal(&mut cursor, metadata).unwrap());
    }

    #[test]
    fn parse_negative_number() {
        // decimal(65,10), column = '-1234567890112233445566778899001112223334445556667778889.9900011112'
        let payload: Vec<u8> = vec![
            65, 10, 126, 242, 4, 199, 45, 249, 79, 116, 26, 222, 55, 163, 236, 255, 239, 7, 96,
            236, 16, 196, 11, 216, 50, 128, 182, 196, 253, 200, 40, 253,
        ];
        let mut cursor = Cursor::new(payload.as_slice());
        let metadata = cursor.read_u16::<LittleEndian>().unwrap();

        let expected =
            String::from("-1234567890112233445566778899001112223334445556667778889.9900011112");
        assert_eq!(expected, parse_decimal(&mut cursor, metadata).unwrap());
    }

    #[test]
    fn parse_with_starting_zeros_ignored() {
        // decimal(65,10), column = '7778889.9900011112'
        let payload: Vec<u8> = vec![
            65, 10, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 118, 178,
            73, 59, 2, 55, 215, 2,
        ];
        let mut cursor = Cursor::new(payload.as_slice());
        let metadata = cursor.read_u16::<LittleEndian>().unwrap();

        let expected = String::from("7778889.9900011112");
        assert_eq!(expected, parse_decimal(&mut cursor, metadata).unwrap());
    }

    #[test]
    fn parse_with_integral_zero() {
        // decimal(65,10), column = '.9900011112'
        let payload: Vec<u8> = vec![
            65, 10, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            59, 2, 55, 215, 2,
        ];
        let mut cursor = Cursor::new(payload.as_slice());
        let metadata = cursor.read_u16::<LittleEndian>().unwrap();

        let expected = String::from("0.9900011112");
        assert_eq!(expected, parse_decimal(&mut cursor, metadata).unwrap());
    }

    #[test]
    fn compressed_fractional_starting_zeros_preserved() {
        // In this test first two zeros are preserved->[uncompr][comp]
        // decimal(60,15), column = '34445556667778889.123456789006700'
        let payload: Vec<u8> = vec![
            60, 15, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 13, 152, 244, 39, 205, 127, 73, 7, 91,
            205, 21, 0, 26, 44,
        ];
        let mut cursor = Cursor::new(payload.as_slice());
        let metadata = cursor.read_u16::<LittleEndian>().unwrap();

        let expected = String::from("34445556667778889.123456789006700");
        assert_eq!(expected, parse_decimal(&mut cursor, metadata).unwrap());
    }

    #[test]
    fn parse_integer() {
        // decimal(60,0), column = '34445556667778889'
        let payload: Vec<u8> = vec![
            60, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 13, 152, 244, 39,
            205, 127, 73,
        ];
        let mut cursor = Cursor::new(payload.as_slice());
        let metadata = cursor.read_u16::<LittleEndian>().unwrap();

        let expected = String::from("34445556667778889");
        assert_eq!(expected, parse_decimal(&mut cursor, metadata).unwrap());
    }
}
