use crate::events::binlog_event::BinlogEvent;
use crate::events::event_header::EventHeader;
use crate::events::event_parser::EventParser;
use crate::replica_options::ReplicaOptions;
use crate::ssl_mode::SslMode;

/// MySql replication client streaming binlog events in real-time.
pub struct BinlogClient {
    pub options: ReplicaOptions,
    pub parser: EventParser,
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
}

impl Iterator for BinlogClient {
    type Item = (EventHeader, BinlogEvent);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
