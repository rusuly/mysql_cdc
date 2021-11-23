use crate::errors::Error;
use crate::events::row_events::mysql_value::{Date, DateTime, Time};
use crate::extensions::{read_bitmap_big_endian, read_string};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

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
