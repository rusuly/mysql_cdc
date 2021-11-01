use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{self, Cursor};

use crate::constants::capability_flags;

/// SSLRequest packet used in SSL/TLS connection.
/// <a href="https://mariadb.com/kb/en/library/connection/#sslrequest-packet">See more</a>
pub struct SslRequestCommand {
    pub client_capabilities: u32,
    pub max_packet_size: u32,
    pub client_collation: u8,
}

impl SslRequestCommand {
    pub fn new(client_collation: u8) -> Self {
        let client_capabilities = capability_flags::LONG_FLAG
            | capability_flags::PROTOCOL_41
            | capability_flags::SECURE_CONNECTION
            | capability_flags::SSL
            | capability_flags::PLUGIN_AUTH;

        let client_capabilities = client_capabilities as u32;

        Self {
            client_capabilities,
            max_packet_size: 0,
            client_collation,
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, io::Error> {
        let mut vec = Vec::new();
        let mut cursor = Cursor::new(&mut vec);

        cursor.write_u32::<LittleEndian>(self.client_capabilities)?;
        cursor.write_u32::<LittleEndian>(self.max_packet_size)?;
        cursor.write_u8(self.client_collation)?;

        // Fill reserved bytes
        for _number in 0..23 {
            cursor.write_u8(0)?;
        }

        Ok(vec)
    }
}
