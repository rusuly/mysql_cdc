use crate::constants::checksum_type::ChecksumType;
use crate::constants::database_provider::DatabaseProvider;
use crate::constants::EVENT_HEADER_SIZE;
use crate::errors::Error;
use crate::events::binlog_event::BinlogEvent;
use crate::events::event_header::EventHeader;
use crate::events::event_parser::EventParser;
use crate::packet_channel::PacketChannel;
use crate::providers::mariadb::mariadb_provider::replicate_mariadb;
use crate::providers::mysql::mysql_provider::replicate_mysql;
use crate::replica_options::ReplicaOptions;
use crate::responses::error_packet::ErrorPacket;
use crate::responses::response_type::ResponseType;
use crate::ssl_mode::SslMode;

/// MySql replication client streaming binlog events in real-time.
pub struct BinlogClient {
    pub options: ReplicaOptions,
}

impl BinlogClient {
    pub fn new(options: ReplicaOptions) -> Self {
        if options.ssl_mode != SslMode::Disabled {
            unimplemented!("Ssl encryption is not supported in this version");
        }

        Self { options }
    }

    /// Replicates binlog events from the server
    pub fn replicate(mut self) -> Result<BinlogEvents, Error> {
        let (mut channel, provider) = self.connect()?;

        self.adjust_starting_position(&mut channel)?;
        self.set_master_heartbeat(&mut channel)?;
        let checksum = self.set_master_binlog_checksum(&mut channel)?;

        let server_id = if self.options.blocking {
            self.options.server_id
        } else {
            0
        };

        match provider {
            DatabaseProvider::MariaDB => replicate_mariadb(&mut channel, &self.options, server_id)?,
            DatabaseProvider::MySQL => replicate_mysql(&mut channel, &self.options, server_id)?,
        }

        Ok(BinlogEvents::new(channel, checksum))
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

    pub fn read_event(&mut self, packet: &[u8]) -> Result<(EventHeader, BinlogEvent), Error> {
        let header = EventHeader::parse(&packet[1..])?;
        let event_slice = &packet[1 + EVENT_HEADER_SIZE..];
        let event = self.parser.parse_event(&header, event_slice)?;
        Ok((header, event))
    }

    pub fn read_error(&mut self, packet: &[u8]) -> Result<(EventHeader, BinlogEvent), Error> {
        let error = ErrorPacket::parse(&packet[1..])?;
        Err(Error::String(format!("Event stream error. {:?}", error)))
    }
}

impl Iterator for BinlogEvents {
    type Item = Result<(EventHeader, BinlogEvent), Error>;

    /// Reads binlog event packets from network stream.
    /// <a href="https://mariadb.com/kb/en/3-binlog-network-stream/">See more</a>
    fn next(&mut self) -> Option<Self::Item> {
        let (packet, _) = match self.channel.read_packet() {
            Ok(x) => x,
            Err(e) => return Some(Err(Error::IoError(e))),
        };
        match packet[0] {
            ResponseType::OK => Some(self.read_event(&packet)),
            ResponseType::ERROR => Some(self.read_error(&packet)),
            ResponseType::END_OF_FILE => None,
            _ => Some(Err(Error::String(
                "Unknown network stream status".to_string(),
            ))),
        }
    }
}
