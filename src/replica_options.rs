use crate::binlog_options::BinlogOptions;
use crate::events::event_type::EventType;
use crate::ssl_mode::SslMode;
use std::collections::HashSet;
use std::time::Duration;
/// Settings used to connect to MySQL/MariaDB.
pub struct ReplicaOptions {
    /// Port number to connect. Defaults to 3306.
    pub port: u16,

    /// Hostname to connect. Defaults to "localhost".
    pub hostname: String,

    /// Defines whether SSL/TLS must be used. Defaults to SslMode.DISABLED.
    pub ssl_mode: SslMode,

    /// A database user which is used to register as a database slave.
    /// The user needs to have <c>REPLICATION SLAVE</c>, <c>REPLICATION CLIENT</c> privileges.
    pub username: String,

    /// The password of the user which is used to connect.
    pub password: String,

    /// Default database name specified in Handshake connection.
    /// Has nothing to do with filtering events by database name.
    pub database: Option<String>,

    /// Specifies the slave server id and used only in blocking mode. Defaults to 65535.
    /// <a href="https://dev.mysql.com/doc/refman/8.0/en/mysqlbinlog-server-id.html">See more</a>
    pub server_id: u32,

    /// Specifies whether to stream events or read until last event and then return.
    /// Defaults to true (stream events and wait for new ones).
    pub blocking: bool,

    /// Defines interval of keep alive messages that the master sends to the slave.
    /// Defaults to 30 seconds.
    pub heartbeat_interval: Duration,

    /// Defines the binlog coordinates that replication should start from.
    /// Defaults to BinlogOptions.FromEnd()
    pub binlog: BinlogOptions,

    /// Defines the event type set to be included
    pub included_events: Option<HashSet<EventType>>,
}

impl Default for ReplicaOptions {
    fn default() -> ReplicaOptions {
        ReplicaOptions {
            port: 3306,
            hostname: String::from("localhost"),
            ssl_mode: SslMode::Disabled,
            username: String::new(),
            password: String::new(),
            database: None,
            server_id: 65535,
            blocking: true,
            heartbeat_interval: Duration::from_secs(30),
            binlog: BinlogOptions::from_end(),
            included_events: None,
        }
    }
}
