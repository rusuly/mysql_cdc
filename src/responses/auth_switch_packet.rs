use crate::{errors::Error, extensions::read_null_term_string};
use std::io::Cursor;

/// Authentication Switch Request.
/// <a href="https://mariadb.com/kb/en/library/connection/#authentication-switch-request">See more</a>
#[derive(Debug)]
pub struct AuthPluginSwitchPacket {
    pub auth_plugin_name: String,
    pub auth_plugin_data: String,
}

impl AuthPluginSwitchPacket {
    pub fn parse(packet: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(packet);

        let auth_plugin_name = read_null_term_string(&mut cursor)?;
        let auth_plugin_data = read_null_term_string(&mut cursor)?;

        Ok(Self {
            auth_plugin_name,
            auth_plugin_data,
        })
    }
}
