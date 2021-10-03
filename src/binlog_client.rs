use crate::commands::dump_binlog_command::DumpBinlogCommand;
use crate::constants::checksum_type::ChecksumType;
use crate::constants::EVENT_HEADER_SIZE;
use crate::events::binlog_event::BinlogEvent;
use crate::events::event_header::EventHeader;
use crate::events::event_parser::EventParser;
use crate::packet_channel::PacketChannel;
use crate::replica_options::ReplicaOptions;
use crate::responses::end_of_file_packet::EndOfFilePacket;
use crate::responses::error_packet::ErrorPacket;
use crate::responses::response_type::ResponseType;
use crate::ssl_mode::SslMode;

/// MySql replication client streaming binlog events in real-time.
pub struct BinlogClient {
    pub options: ReplicaOptions,
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

        Self { options }
    }

    /// Replicates binlog events from the server
    pub fn replicate(mut self) -> BinlogEvents {
        let (mut channel, provider) = self.connect();
        self.adjust_starting_position(&mut channel);
        self.set_master_heartbeat(&mut channel);
        let checksum = self.set_master_binlog_checksum(&mut channel);

        self.dump_binlog(&mut channel);
        BinlogEvents::new(channel, checksum)
    }

    pub fn dump_binlog(mut self, channel: &mut PacketChannel) {
        let server_id = if self.options.blocking {
            self.options.server_id
        } else {
            0
        };
        let command = DumpBinlogCommand::new(
            server_id,
            self.options.binlog.filename.clone(),
            self.options.binlog.position,
        );
        channel.write_packet(&command.serialize(), 0)
    }
}

pub struct BinlogEvents {
    pub channel: PacketChannel,
    pub parser: EventParser,
}

impl BinlogEvents {
    pub fn new(channel: PacketChannel, checksum: ChecksumType) -> Self {
        let mut parser = EventParser::new();
        parser.checksum_type = checksum;

        Self { channel, parser }
    }
}

impl Iterator for BinlogEvents {
    type Item = (EventHeader, BinlogEvent);

    /// Reads binlog event packets from network stream.
    /// <a href="https://mariadb.com/kb/en/3-binlog-network-stream/">See more</a>
    fn next(&mut self) -> Option<Self::Item> {
        let (packet, _) = self.channel.read_packet();
        match packet[0] {
            ResponseType::OK => {
                let header = EventHeader::parse(&packet[1..]);
                let event_slice = &packet[1 + EVENT_HEADER_SIZE..];
                let event = self.parser.parse_event(&header, event_slice);
                Some((header, event))
            }
            ResponseType::END_OF_FILE => {
                let _ = EndOfFilePacket::parse(&packet[1..]);
                None
            }
            ResponseType::ERROR => {
                let error = ErrorPacket::parse(&packet[1..]);
                panic!("Event stream error. {:?}", error);
            }
            _ => unreachable!("Unknown network stream status"),
        }
    }
}
