use crate::commands::ssl_request_command::SslRequestCommand;
use crate::constants::database_provider::DatabaseProvider;
use crate::constants::{auth_plugin_names, capability_flags, UTF8_MB4_GENERAL_CI};
use crate::events::binlog_event::BinlogEvent;
use crate::events::event_header::EventHeader;
use crate::events::event_parser::EventParser;
use crate::options::ReplicaOptions;
use crate::packet_channel::PacketChannel;
use crate::responses::error_packet::ErrorPacket;
use crate::responses::handshake_packet::HandshakePacket;
use crate::responses::response_type::ResponseType;
use crate::ssl_mode::SslMode;

/// MySql replication client streaming binlog events in real-time.
pub struct BinlogClient {
    options: ReplicaOptions,
    parser: EventParser,
}

impl BinlogClient {
    pub fn new(options: ReplicaOptions) -> Self {
        if options.ssl_mode == SslMode::REQUIRE_VERIFY_CA
            || options.ssl_mode == SslMode::REQUIRE_VERIFY_FULL
        {
            unimplemented!(
                "{:?} and {:?} ssl modes are not supported",
                SslMode::REQUIRE_VERIFY_CA,
                SslMode::REQUIRE_VERIFY_FULL
            );
        }

        Self {
            options,
            parser: EventParser::new(),
        }
    }

    /// Replicates binlog events from the server
    pub fn replicate(self) -> Self {
        let (channel, provider) = self.connect();
        self
    }

    fn connect(&self) -> (PacketChannel, DatabaseProvider) {
        let mut channel = PacketChannel::new(&self.options);
        let (packet, seq_num) = channel.read_packet();
        self.panic_if_error(&packet, "Initial handshake error.");
        let handshake = HandshakePacket::parse(&packet);

        if handshake.auth_plugin_name != auth_plugin_names::MySqlNativePassword
            && handshake.auth_plugin_name != auth_plugin_names::CachingSha2Password
        {
            unimplemented!(
                "Authentication plugin {} is not supported.",
                handshake.auth_plugin_name
            );
        }

        self.authenticate(&mut channel, &handshake, seq_num);
        (channel, DatabaseProvider::from(&handshake.server_version))
    }

    fn authenticate(
        &self,
        channel: &mut PacketChannel,
        handshake: &HandshakePacket,
        mut seq_num: u8,
    ) {
        let mut use_ssl = false;
        if self.options.ssl_mode != SslMode::DISABLED {
            let ssl_available = (handshake.server_capabilities & capability_flags::SSL) != 0;
            if !ssl_available && self.options.ssl_mode as u8 >= SslMode::REQUIRE as u8 {
                panic!("The server doesn't support SSL encryption");
            }
            if ssl_available {
                let command = SslRequestCommand::new(UTF8_MB4_GENERAL_CI);
                seq_num += 1;
                channel.write_packet(&command.serialize(), seq_num);
                channel.upgrade_to_ssl();
                use_ssl = true;
            }
        }
    }

    fn panic_if_error(&self, packet: &[u8], message: &str) {
        if packet[0] == ResponseType::Error {
            let error = ErrorPacket::parse(&packet[1..]);
            panic!("{} {:?}", message, error)
        }
    }
}

impl Iterator for BinlogClient {
    type Item = (EventHeader, BinlogEvent);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
