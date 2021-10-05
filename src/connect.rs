use openssl::rsa::{Padding, Rsa};

use crate::binlog_client::BinlogClient;
use crate::commands::auth_plugin_switch_command::AuthPluginSwitchCommand;
use crate::commands::authenticate_command::AuthenticateCommand;
use crate::commands::ssl_request_command::SslRequestCommand;
use crate::constants::database_provider::DatabaseProvider;
use crate::constants::{auth_plugin_names, capability_flags, NULL_TERMINATOR, UTF8_MB4_GENERAL_CI};
use crate::extensions::{panic_if_error, xor};
use crate::packet_channel::PacketChannel;
use crate::responses::auth_switch_packet::AuthPluginSwitchPacket;
use crate::responses::handshake_packet::HandshakePacket;
use crate::responses::response_type::ResponseType;
use crate::ssl_mode::SslMode;

impl BinlogClient {
    pub fn connect(&self) -> (PacketChannel, DatabaseProvider) {
        let mut channel = PacketChannel::new(&self.options);
        let (packet, seq_num) = channel.read_packet();
        panic_if_error(&packet, "Initial handshake error.");
        let handshake = HandshakePacket::parse(&packet);

        self.check_auth_plugin(&handshake.auth_plugin_name);
        self.authenticate(&mut channel, &handshake, seq_num + 1);
        (channel, DatabaseProvider::from(&handshake.server_version))
    }

    fn authenticate(
        &self,
        channel: &mut PacketChannel,
        handshake: &HandshakePacket,
        mut seq_num: u8,
    ) {
        let mut use_ssl = false;
        if self.options.ssl_mode != SslMode::Disabled {
            let ssl_available = (handshake.server_capabilities & capability_flags::SSL) != 0;
            if !ssl_available && self.options.ssl_mode as u8 >= SslMode::Require as u8 {
                panic!("The server doesn't support SSL encryption");
            }
            if ssl_available {
                let ssl_command = SslRequestCommand::new(UTF8_MB4_GENERAL_CI);
                channel.write_packet(&ssl_command.serialize(), seq_num);
                seq_num += 1;
                channel.upgrade_to_ssl();
                use_ssl = true;
            }
        }

        let auth_command = AuthenticateCommand::new(&self.options, handshake, UTF8_MB4_GENERAL_CI);
        channel.write_packet(&auth_command.serialize(), seq_num);
        let (packet, seq_num) = channel.read_packet();
        panic_if_error(&packet, "Authentication error.");

        match packet[0] {
            ResponseType::OK => return,
            ResponseType::AUTH_PLUGIN_SWITCH => {
                let switch_packet = AuthPluginSwitchPacket::parse(&packet[1..]);
                self.handle_auth_plugin_switch(channel, switch_packet, seq_num + 1, use_ssl);
            }
            _ => {
                self.authenticate_sha_256(
                    channel,
                    &packet,
                    &handshake.scramble,
                    seq_num + 1,
                    use_ssl,
                );
            }
        }
    }

    fn handle_auth_plugin_switch(
        &self,
        channel: &mut PacketChannel,
        switch_packet: AuthPluginSwitchPacket,
        seq_num: u8,
        use_ssl: bool,
    ) {
        self.check_auth_plugin(&switch_packet.auth_plugin_name);

        let auth_switch_command = AuthPluginSwitchCommand::new(
            &self.options.password,
            &switch_packet.auth_plugin_data,
            &switch_packet.auth_plugin_name,
        );
        channel.write_packet(&auth_switch_command.serialize(), seq_num);
        let (packet, seq_num) = channel.read_packet();
        panic_if_error(&packet, "Authentication switch error.");

        if switch_packet.auth_plugin_name == auth_plugin_names::CACHING_SHA2_PASSWORD {
            self.authenticate_sha_256(
                channel,
                &packet,
                &switch_packet.auth_plugin_data,
                seq_num + 1,
                use_ssl,
            );
        }
    }

    fn authenticate_sha_256(
        &self,
        channel: &mut PacketChannel,
        packet: &[u8],
        scramble: &String,
        seq_num: u8,
        use_ssl: bool,
    ) {
        // See https://mariadb.com/kb/en/caching_sha2_password-authentication-plugin/
        // Success authentication.
        if packet[0] == 0x01 && packet[1] == 0x03 {
            return;
        }

        let mut password = self.options.password.as_bytes().to_vec();
        password.push(NULL_TERMINATOR);

        // Send clear password if ssl is used.
        if use_ssl {
            channel.write_packet(&password, seq_num);
            let (packet, _seq_num) = channel.read_packet();
            panic_if_error(&packet, "Sending clear password error.");
            return;
        }

        // Request public key.
        channel.write_packet(&[0x02], seq_num);
        let (packet, seq_num) = channel.read_packet();
        panic_if_error(&packet, "Requesting caching_sha2_password public key.");

        // Extract public key.
        let public_key = &packet[1..];
        let encrypted_password = xor(&password, &scramble.as_bytes());

        let rsa = Rsa::public_key_from_pem(public_key).unwrap();
        let mut encrypted_body = vec![0u8; rsa.size() as usize];
        rsa.public_encrypt(
            &encrypted_password,
            &mut encrypted_body,
            Padding::PKCS1_OAEP,
        )
        .unwrap();

        channel.write_packet(&encrypted_body, seq_num + 1);
        let (packet, _seq_num) = channel.read_packet();
        panic_if_error(&packet, "Authentication error.");
    }

    fn check_auth_plugin(&self, auth_plugin_name: &String) {
        if auth_plugin_name != auth_plugin_names::MY_SQL_NATIVE_PASSWORD
            && auth_plugin_name != auth_plugin_names::CACHING_SHA2_PASSWORD
        {
            unimplemented!(
                "Authentication plugin {} is not supported.",
                auth_plugin_name
            );
        }
    }
}
