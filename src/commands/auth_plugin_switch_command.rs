use crate::extensions::encrypt_password;
use std::io::{Cursor, Write};

pub struct AuthPluginSwitchCommand {
    pub password: String,
    pub scramble: String,
    pub auth_plugin_name: String,
}

impl AuthPluginSwitchCommand {
    pub fn new(password: &String, scramble: &String, auth_plugin_name: &String) -> Self {
        Self {
            password: password.clone(),
            scramble: scramble.clone(),
            auth_plugin_name: auth_plugin_name.clone(),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        let mut cursor = Cursor::new(&mut vec);

        let encrypted_password =
            encrypt_password(&self.password, &self.scramble, &self.auth_plugin_name);
        cursor.write(&encrypted_password).unwrap();

        vec
    }
}
