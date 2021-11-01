use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{self, Cursor, Write};

use crate::constants::auth_plugin_names::AuthPlugin;
use crate::constants::capability_flags;
use crate::extensions::{encrypt_password, write_null_term_string};
use crate::replica_options::ReplicaOptions;
use crate::responses::handshake_packet::HandshakePacket;

/// Client handshake response to the server initial handshake packet.
/// <a href="https://mariadb.com/kb/en/library/connection/#handshake-response-packet">See more</a>
pub struct AuthenticateCommand {
    pub client_capabilities: u32,
    pub max_packet_size: u32,
    pub client_collation: u8,
    pub username: String,
    pub password: String,
    pub database: Option<String>,
    pub scramble: String,
    pub auth_plugin: AuthPlugin,
    pub auth_plugin_name: String,
}

impl AuthenticateCommand {
    pub fn new(
        options: &ReplicaOptions,
        handshake: &HandshakePacket,
        auth_plugin: AuthPlugin,
        client_collation: u8,
    ) -> Self {
        let mut client_capabilities = capability_flags::LONG_FLAG
            | capability_flags::PROTOCOL_41
            | capability_flags::SECURE_CONNECTION
            | capability_flags::PLUGIN_AUTH;

        if let Some(_x) = &options.database {
            client_capabilities |= capability_flags::CONNECT_WITH_DB;
        }

        let client_capabilities = client_capabilities as u32;

        Self {
            client_capabilities,
            max_packet_size: 0,
            client_collation,
            username: options.username.clone(),
            password: options.password.clone(),
            database: options.database.clone(),
            scramble: handshake.scramble.clone(),
            auth_plugin_name: handshake.auth_plugin_name.clone(),
            auth_plugin: auth_plugin,
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

        write_null_term_string(&mut cursor, &self.username)?;

        let encrypted_password =
            encrypt_password(&self.password, &self.scramble, &self.auth_plugin);
        cursor.write_u8(encrypted_password.len() as u8)?;
        cursor.write(&encrypted_password)?;

        if let Some(database) = &self.database {
            write_null_term_string(&mut cursor, database)?;
        }

        write_null_term_string(&mut cursor, &self.auth_plugin_name)?;
        Ok(vec)
    }
}
