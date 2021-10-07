//! # mysql_cdc
//! MySQL/MariaDB binlog replication client for Rust
//!
//! ## Limitations
//! Please note the lib currently has the following limitations:
//! - Supports only standard auth plugins `mysql_native_password` and `caching_sha2_password`.
//! - **Currently, the library doesn't support SSL encryption.**
//! - **GTID replication is work in progress.**
//! - **Doesn't handle split packets (16MB and more).**
//!
//! ## Binlog event stream replication
//! Real-time replication client works the following way.
//! ```no_run
//! use mysql_cdc::binlog_client::BinlogClient;
//! use mysql_cdc::binlog_options::BinlogOptions;
//! use mysql_cdc::replica_options::ReplicaOptions;
//! use mysql_cdc::ssl_mode::SslMode;
//!
//! let options = ReplicaOptions {
//!     username: String::from("root"),
//!     password: String::from("Qwertyu1"),
//!     blocking: false,
//!     ssl_mode: SslMode::Disabled,
//!     binlog: BinlogOptions::from_start(),
//!     ..Default::default()
//! };
//!
//! let client = BinlogClient::new(options);
//!
//! for (header, binlog_event) in client.replicate() {
//!     println!("{:#?}", header);
//!     println!("{:#?}", binlog_event);
//! }
//! ```
//! A typical transaction has the following structure.
//! 1. `GtidEvent` if gtid mode is enabled.
//! 2. One or many `TableMapEvent` events.
//!    - One or many `WriteRowsEvent` events.
//!    - One or many `UpdateRowsEvent` events.
//!    - One or many `DeleteRowsEvent` events.
//! 3. `XidEvent` indicating commit of the transaction.
//!
//! **It's best practice to use GTID replication with the `from_gtid` method.** Using the approach you can correctly perform replication failover.
//! Note that in GTID mode `from_gtid` has the following behavior:
//! - `from_gtid(@@gtid_purged)` acts like `from_start()`
//! - `from_gtid(@@gtid_executed)` acts like `from_end()`
//!
//! ## Reading binlog files offline
//! In some cases you will need to read binlog files offline from the file system.
//! This can be done using `BinlogReader` class.
//! ```no_run
//! use std::fs::File;
//! use mysql_cdc::binlog_reader::BinlogReader;
//!
//! let file = File::open("mysql-bin.000001").unwrap();
//! let reader = BinlogReader::new(file).unwrap();
//!
//! for (header, binlog_event) in reader.read_events() {
//!     println!("{:#?}", header);
//!     println!("{:#?}", binlog_event);
//! }
//! ```

pub mod binlog_client;
pub mod binlog_options;
pub mod binlog_reader;
pub mod events;
pub mod metadata;
pub mod replica_options;
pub mod ssl_mode;
pub mod starting_strategy;

mod commands;
mod configure;
mod connect;
mod constants;
mod extensions;
mod packet_channel;
mod providers;
mod responses;
