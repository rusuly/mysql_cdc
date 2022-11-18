use crate::binlog_events::BinlogEvents;
use crate::binlog_raw_events::BinlogRawEvents;
use crate::constants::database_provider::DatabaseProvider;
use crate::errors::Error;
use crate::events::binlog_event::BinlogEvent;
use crate::events::event_header::EventHeader;
use crate::providers::mariadb::gtid::gtid::Gtid as MariaGtid;
use crate::providers::mariadb::mariadb_provider::replicate_mariadb;
use crate::providers::mysql::gtid::gtid::Gtid as MySqlGtid;
use crate::providers::mysql::mysql_provider::replicate_mysql;
use crate::replica_options::ReplicaOptions;
use crate::ssl_mode::SslMode;
use crate::starting_strategy::StartingStrategy;

/// MySql replication client streaming binlog events in real-time.
pub struct BinlogClient {
    pub options: ReplicaOptions,
    transaction: bool,
    maria_gtid: Option<MariaGtid>,
    mysql_gtid: Option<MySqlGtid>,
}

impl BinlogClient {
    pub fn new(options: ReplicaOptions) -> Self {
        if options.ssl_mode != SslMode::Disabled {
            unimplemented!("Ssl encryption is not supported in this version");
        }

        Self {
            options,
            transaction: false,
            maria_gtid: None,
            mysql_gtid: None,
        }
    }

    pub fn replicate_raw(&mut self) -> Result<(BinlogRawEvents, ChecksumType), Error> {
        let (mut channel, provider) = self.connect()?;

        // Reset on reconnect
        self.transaction = false;
        self.maria_gtid = None;
        self.mysql_gtid = None;

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
        Ok((BinlogRawEvents::new(channel), checksum))
    }

    /// Replicates binlog events from the server
    pub fn replicate(&mut self) -> Result<BinlogEvents, Error> {
        let (mut channel, provider) = self.connect()?;

        // Reset on reconnect
        self.transaction = false;
        self.maria_gtid = None;
        self.mysql_gtid = None;

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

    /// Updates current replication position
    pub fn commit(&mut self, header: &EventHeader, event: &BinlogEvent) {
        self.update_gtid_position(event);
        self.update_binlog_position(header, event);
    }

    fn update_gtid_position(&mut self, event: &BinlogEvent) {
        if self.options.binlog.starting_strategy != StartingStrategy::FromGtid {
            return;
        }

        match event {
            BinlogEvent::MariaDbGtidEvent(x) => {
                self.maria_gtid = Some(x.gtid.clone());
            }
            BinlogEvent::MySqlGtidEvent(x) => {
                self.mysql_gtid = Some(x.gtid.clone());
            }
            BinlogEvent::XidEvent(_) => {
                self.commit_gtid();
            }
            BinlogEvent::QueryEvent(x) => {
                if x.sql_statement.is_empty() {
                    return;
                }
                if x.sql_statement == "BEGIN" {
                    self.transaction = true;
                } else if x.sql_statement == "COMMIT" || x.sql_statement == "ROLLBACK" {
                    self.commit_gtid();
                } else if !self.transaction {
                    // Auto-commit query like DDL
                    self.commit_gtid();
                }
            }
            _ => {}
        }
    }

    fn update_binlog_position(&mut self, header: &EventHeader, event: &BinlogEvent) {
        // Rows event depends on preceding TableMapEvent & we change the position
        // after we read them atomically to prevent missing mapping on reconnect.
        // Figure out something better as TableMapEvent can be followed by several row events.
        match event {
            BinlogEvent::TableMapEvent(_) => return,
            BinlogEvent::RotateEvent(x) => {
                self.options.binlog.filename = x.binlog_filename.clone();
                self.options.binlog.position = x.binlog_position as u32;
            }
            _ => {
                if header.next_event_position > 0 {
                    self.options.binlog.position = header.next_event_position;
                }
            }
        }
    }

    fn commit_gtid(&mut self) {
        self.transaction = false;

        if let Some(gtid) = &self.maria_gtid {
            if let Some(list) = &mut self.options.binlog.gtid_list {
                list.add_gtid(gtid.clone());
            }
        }
        if let Some(gtid) = &self.mysql_gtid {
            if let Some(set) = &mut self.options.binlog.gtid_set {
                set.add_gtid(gtid.clone()).unwrap();
            }
        }
    }
}
