use crate::errors::Error;
use crate::events::row_events::mysql_value::{Date, DateTime, Time};
use crate::extensions::{read_bitmap_big_endian, read_string};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
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

pub fn parse_string(cursor: &mut Cursor<&[u8]>, metadata: u16) -> Result<String, Error> {
    let length = if metadata < 256 {
        cursor.read_u8()? as usize
    } else {
        cursor.read_u16::<LittleEndian>()? as usize
    };
    Ok(read_string(cursor, length)?)
}

pub fn parse_bit(cursor: &mut Cursor<&[u8]>, metadata: u16) -> Result<Vec<bool>, Error> {
    let length = (metadata >> 8) * 8 + (metadata & 0xFF);
    let mut bitmap = read_bitmap_big_endian(cursor, length as usize)?;
    bitmap.reverse();
    Ok(bitmap)
}

pub fn parse_blob(cursor: &mut Cursor<&[u8]>, metadata: u16) -> Result<Vec<u8>, Error> {
    let length = cursor.read_uint::<LittleEndian>(metadata as usize)? as usize;
    let mut vec = vec![0; length];
    cursor.read_exact(&mut vec)?;
    Ok(vec)
}

pub fn parse_year(cursor: &mut Cursor<&[u8]>, _metadata: u16) -> Result<u16, Error> {
    Ok(1900 + cursor.read_u8()? as u16)
}

pub fn parse_date(cursor: &mut Cursor<&[u8]>, _metadata: u16) -> Result<Date, Error> {
    let value = cursor.read_u24::<LittleEndian>()?;

    // Bits 1-5 store the day. Bits 6-9 store the month. The remaining bits store the year.
    let day = value % (1 << 5);
    let month = (value >> 5) % (1 << 4);
    let year = value >> 9;

    Ok(Date {
        year: year as u16,
        month: month as u8,
        day: day as u8,
    })
}

pub fn parse_time(cursor: &mut Cursor<&[u8]>, _metadata: u16) -> Result<Time, Error> {
    let mut value = (cursor.read_i24::<LittleEndian>()? << 8) >> 8;

    if value < 0 {
        return Err(Error::String(
            "Parsing negative TIME values is not supported in this version".to_string(),
        ));
    }

    let second = value % 100;
    value = value / 100;
    let minute = value % 100;
    value = value / 100;
    let hour = value;
    Ok(Time {
        hour: hour as i16,
        minute: minute as u8,
        second: second as u8,
        millis: 0,
    })
}

pub fn parse_time2(cursor: &mut Cursor<&[u8]>, metadata: u16) -> Result<Time, Error> {
    let value = cursor.read_u24::<BigEndian>()?;
    let millis = parse_fractional_part(cursor, metadata)? / 1000;

    let negative = ((value >> 23) & 1) == 0;
    if negative {
        // It looks like other similar clients don't parse TIME2 values properly
        // In negative time values both TIME and FSP are stored in reverse order
        // See https://github.com/mysql/mysql-server/blob/ea7d2e2d16ac03afdd9cb72a972a95981107bf51/sql/log_event.cc#L2022
        // See https://github.com/mysql/mysql-server/blob/ea7d2e2d16ac03afdd9cb72a972a95981107bf51/mysys/my_time.cc#L1784
        return Err(Error::String(
            "Parsing negative TIME values is not supported in this version".to_string(),
        ));
    }

    // 1 bit sign. 1 bit unused. 10 bits hour. 6 bits minute. 6 bits second.
    let hour = (value >> 12) % (1 << 10);
    let minute = (value >> 6) % (1 << 6);
    let second = value % (1 << 6);

    Ok(Time {
        hour: hour as i16,
        minute: minute as u8,
        second: second as u8,
        millis: millis as u32,
    })
}

pub fn parse_date_time(cursor: &mut Cursor<&[u8]>, _metadata: u16) -> Result<DateTime, Error> {
    let mut value = cursor.read_u64::<LittleEndian>()?;
    let second = value % 100;
    value = value / 100;
    let minute = value % 100;
    value = value / 100;
    let hour = value % 100;
    value = value / 100;
    let day = value % 100;
    value = value / 100;
    let month = value % 100;
    value = value / 100;
    let year = value;

    Ok(DateTime {
        year: year as u16,
        month: month as u8,
        day: day as u8,
        hour: hour as u8,
        minute: minute as u8,
        second: second as u8,
        millis: 0,
    })
}

pub fn parse_date_time2(cursor: &mut Cursor<&[u8]>, metadata: u16) -> Result<DateTime, Error> {
    let value = cursor.read_uint::<BigEndian>(5)?;
    let millis = parse_fractional_part(cursor, metadata)? / 1000;

    // 1 bit sign(always true). 17 bits year*13+month. 5 bits day. 5 bits hour. 6 bits minute. 6 bits second.
    let year_month = (value >> 22) % (1 << 17);
    let year = year_month / 13;
    let month = year_month % 13;
    let day = (value >> 17) % (1 << 5);
    let hour = (value >> 12) % (1 << 5);
    let minute = (value >> 6) % (1 << 6);
    let second = value % (1 << 6);

    Ok(DateTime {
        year: year as u16,
        month: month as u8,
        day: day as u8,
        hour: hour as u8,
        minute: minute as u8,
        second: second as u8,
        millis: millis as u32,
    })
}

pub fn parse_timestamp(cursor: &mut Cursor<&[u8]>, _metadata: u16) -> Result<u64, Error> {
    let seconds = cursor.read_u32::<LittleEndian>()? as u64;
    Ok(seconds * 1000)
}

pub fn parse_timestamp2(cursor: &mut Cursor<&[u8]>, metadata: u16) -> Result<u64, Error> {
    let seconds = cursor.read_u32::<BigEndian>()? as u64;
    let millisecond = parse_fractional_part(cursor, metadata)? / 1000;
    let timestamp = seconds * 1000 + millisecond;
    Ok(timestamp)
}

fn parse_fractional_part(cursor: &mut Cursor<&[u8]>, metadata: u16) -> Result<u64, Error> {
    let length = (metadata + 1) / 2;
    if length == 0 {
        return Ok(0);
    }

    let fraction = cursor.read_uint::<BigEndian>(length as usize)?;
    Ok(fraction * u64::pow(100, 3 - length as u32))
}
