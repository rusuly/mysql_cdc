use crate::{constants::auth_plugin_names::AuthPlugin, extensions::encrypt_password};
use std::io::{self, Cursor, Write};

pub struct AuthPluginSwitchCommand {
    pub password: String,
    pub scramble: String,
    pub auth_plugin_name: String,
    pub auth_plugin: AuthPlugin,
}

impl AuthPluginSwitchCommand {
    pub fn new(
        password: &String,
        scramble: &String,
        auth_plugin_name: &String,
        auth_plugin: AuthPlugin,
    ) -> Self {
        Self {
            password: password.clone(),
            scramble: scramble.clone(),
            auth_plugin_name: auth_plugin_name.clone(),
            auth_plugin: auth_plugin,
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, io::Error> {
        let mut vec = Vec::new();
        let mut cursor = Cursor::new(&mut vec);

        let encrypted_password =
            encrypt_password(&self.password, &self.scramble, &self.auth_plugin);
        cursor.write(&encrypted_password)?;

        Ok(vec)
    }
}
