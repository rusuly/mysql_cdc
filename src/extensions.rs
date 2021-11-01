use crate::constants::auth_plugin_names::AuthPlugin;
use crate::constants::NULL_TERMINATOR;
use crate::errors::Error;
use crate::responses::error_packet::ErrorPacket;
use crate::responses::response_type::ResponseType;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::io::{self, BufRead, Cursor, Read, Write};

pub fn encrypt_password(password: &String, scramble: &String, auth_plugin: &AuthPlugin) -> Vec<u8> {
    match auth_plugin {
        AuthPlugin::MySqlNativePassword => {
            let password_hash = sha1(password.as_bytes());
            let concat_hash = [scramble.as_bytes().to_vec(), sha1(&password_hash)].concat();
            xor(&password_hash, &sha1(&concat_hash))
        }
        AuthPlugin::CachingSha2Password => {
            let password_hash = sha256(password.as_bytes());
            let concat_hash = [scramble.as_bytes().to_vec(), sha256(&password_hash)].concat();
            xor(&password_hash, &sha256(&concat_hash))
        }
    }
}

pub fn sha1(value: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(value);
    hasher.finalize().as_slice().to_vec()
}

pub fn sha256(value: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(value);
    hasher.finalize().as_slice().to_vec()
}

pub fn xor(slice1: &[u8], slice2: &[u8]) -> Vec<u8> {
    let mut result = vec![0u8; slice1.len()];
    for i in 0..result.len() {
        result[i] = slice1[i] ^ slice2[i % slice2.len()];
    }
    result
}

pub fn read_null_term_string(cursor: &mut Cursor<&[u8]>) -> Result<String, Error> {
    let mut vec = Vec::new();
    cursor.read_until(NULL_TERMINATOR, &mut vec)?;
    vec.pop();
    Ok(String::from_utf8(vec)?)
}

pub fn write_null_term_string(
    cursor: &mut Cursor<&mut Vec<u8>>,
    str: &String,
) -> Result<(), io::Error> {
    cursor.write(str.as_bytes())?;
    cursor.write_u8(NULL_TERMINATOR)?;
    Ok(())
}

pub fn read_string(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<String, Error> {
    let mut vec = vec![0; size];
    cursor.read_exact(&mut vec)?;
    Ok(String::from_utf8(vec)?)
}

pub fn read_len_enc_str(cursor: &mut Cursor<&[u8]>) -> Result<String, Error> {
    let length = read_len_enc_num(cursor)?;
    Ok(read_string(cursor, length)?)
}

/// if first byte is less than 0xFB - Integer value is this 1 byte integer
/// 0xFB - NULL value
/// 0xFC - Integer value is encoded in the next 2 bytes (3 bytes total)
/// 0xFD - Integer value is encoded in the next 3 bytes (4 bytes total)
/// 0xFE - Integer value is encoded in the next 8 bytes (9 bytes total)
pub fn read_len_enc_num(cursor: &mut Cursor<&[u8]>) -> Result<usize, Error> {
    let first_byte = cursor.read_u8()?;

    if first_byte < 0xFB {
        Ok(first_byte as usize)
    } else if first_byte == 0xFB {
        Err(Error::String(
            "Length encoded integer cannot be NULL.".to_string(),
        ))
    } else if first_byte == 0xFC {
        Ok(cursor.read_u16::<LittleEndian>()? as usize)
    } else if first_byte == 0xFD {
        Ok(cursor.read_u24::<LittleEndian>()? as usize)
    } else if first_byte == 0xFE {
        Ok(cursor.read_u64::<LittleEndian>()? as usize)
    } else {
        let value = format!("Unexpected length-encoded integer: {}", first_byte).to_string();
        Err(Error::String(value))
    }
}

/// Reads bitmap in little-endian bytes order
pub fn read_bitmap_little_endian(
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
            result[index] = (value & (1 << y)) > 0;
        }
    }
    Ok(result)
}

/// Reads bitmap in big-endian bytes order
pub fn read_bitmap_big_endian(
    cursor: &mut Cursor<&[u8]>,
    bits_number: usize,
) -> Result<Vec<bool>, io::Error> {
    let mut result = vec![false; bits_number];
    let bytes_number = (bits_number + 7) / 8;
    for i in 0..bytes_number {
        let value = cursor.read_u8()?;
        for y in 0..8 {
            let index = ((bytes_number - i - 1) << 3) + y;
            if index >= bits_number {
                continue;
            }
            result[index] = (value & (1 << y)) > 0;
        }
    }
    Ok(result)
}

pub fn check_error_packet(packet: &[u8], message: &str) -> Result<(), Error> {
    if packet[0] == ResponseType::ERROR {
        let error = ErrorPacket::parse(&packet[1..])?;
        let message = format!("{} {:?}", message, error).to_string();
        return Err(Error::String(message));
    }
    return Ok(());
}
