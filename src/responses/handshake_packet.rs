use crate::constants::capability_flags;
use crate::errors::Error;
use crate::extensions::{read_null_term_string, read_string};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

/// Initial handshake packet sent by the server.
/// <a href="https://mariadb.com/kb/en/library/connection/#initial-handshake-packet">See more</a>
#[derive(Debug)]
pub struct HandshakePacket {
    pub protocol_version: u8,
    pub server_version: String,
    pub connection_id: u32,
    pub scramble: String,
    pub server_capabilities: u64,
    pub server_collation: u8,
    pub status_flags: u16,
    pub filler: String,
    pub auth_plugin_length: u8,
    pub auth_plugin_name: String,
}

impl HandshakePacket {
    pub fn parse(packet: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(packet);

        let protocol_version = cursor.read_u8()?;
        let server_version = read_null_term_string(&mut cursor)?;
        let connection_id = cursor.read_u32::<LittleEndian>()?;
        let mut scramble = read_null_term_string(&mut cursor)?;

        let mut capability_flags_1 = vec![0u8; 2];
        cursor.read_exact(&mut capability_flags_1)?;

        let server_collation = cursor.read_u8()?;
        let status_flags = cursor.read_u16::<LittleEndian>()?;

        let mut capability_flags_2 = vec![0u8; 2];
        cursor.read_exact(&mut capability_flags_2)?;

        let auth_plugin_length = cursor.read_u8()?;
        let filler = read_string(&mut cursor, 6)?;

        let mut capability_flags_3 = vec![0u8; 4];
        cursor.read_exact(&mut capability_flags_3)?;

        // Join lower and upper capability flags to a number
        let capability_flags =
            [capability_flags_1, capability_flags_2, capability_flags_3].concat();

        let server_capabilities = Cursor::new(&capability_flags).read_u64::<LittleEndian>()?;

        // Handle specific conditions
        if (server_capabilities & capability_flags::SECURE_CONNECTION) > 0 {
            scramble += &read_null_term_string(&mut cursor)?;
        }

        let mut auth_plugin_name = String::new();
        if (server_capabilities & capability_flags::PLUGIN_AUTH) > 0 {
            auth_plugin_name = read_null_term_string(&mut cursor)?;
        }

        Ok(Self {
            protocol_version,
            server_version,
            connection_id,
            scramble,
            server_capabilities,
            server_collation,
            status_flags,
            filler,
            auth_plugin_length,
            auth_plugin_name,
        })
    }
}
