//! # mysql_cdc
//! MySQL/MariaDB binlog replication client for Rust
//!
//! ## Limitations
//! Please note the lib currently has the following limitations:
//! - Supports only standard auth plugins `mysql_native_password` and `caching_sha2_password`.
//! - **Currently, the library doesn't support SSL encryption.**
//! - **Doesn't handle split packets (16MB and more).**
//!
//! ## Binlog event stream replication
//! Real-time replication client works the following way.
//! ```no_run
//! use mysql_cdc::binlog_client::BinlogClient;
//! use mysql_cdc::binlog_options::BinlogOptions;
//! use mysql_cdc::errors::Error;
//! use mysql_cdc::providers::mariadb::gtid::gtid_list::GtidList;
//! use mysql_cdc::providers::mysql::gtid::gtid_set::GtidSet;
//! use mysql_cdc::replica_options::ReplicaOptions;
//! use mysql_cdc::ssl_mode::SslMode;
//!
//! fn main() -> Result<(), Error> {
//!     // Start replication from MariaDB GTID
//!     let _options = BinlogOptions::from_mariadb_gtid(GtidList::parse("0-1-270")?);
//!
//!     // Start replication from MySQL GTID
//!     let gtid_set =
//!         "d4c17f0c-4f11-11ea-93e3-325d3e1cd1c8:1-107, f442510a-2881-11ea-b1dd-27916133dbb2:1-7";
//!     let _options = BinlogOptions::from_mysql_gtid(GtidSet::parse(gtid_set)?);
//!
//!     // Start replication from the position
//!     let _options = BinlogOptions::from_position(String::from("mysql-bin.000008"), 195);
//!
//!     // Start replication from last master position.
//!     // Useful when you are only interested in new changes.
//!     let _options = BinlogOptions::from_end();
//!
//!     // Start replication from first event of first available master binlog.
//!     // Note that binlog files by default have expiration time and deleted.
//!     let options = BinlogOptions::from_start();
//!
//!     let options = ReplicaOptions {
//!         username: String::from("root"),
//!         password: String::from("Qwertyu1"),
//!         blocking: true,
//!         ssl_mode: SslMode::Disabled,
//!         binlog: options,
//!         ..Default::default()
//!     };
//!
//!     let mut client = BinlogClient::new(options);
//!
//!     for result in client.replicate()? {
//!         let (header, event) = result?;
//!         println!("{:#?}", header);
//!         println!("{:#?}", event);
//!
//!         // You process an event here
//!
//!         // After you processed the event, you need to update replication position
//!         client.commit(&header, &event);
//!     }
//!     Ok(())
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
//! use mysql_cdc::{binlog_reader::BinlogReader, errors::Error};
//! use std::fs::File;
//!
//! const PATH: &str = "mysql-bin.000001";
//!
//! fn main() -> Result<(), Error> {
//!     let file = File::open(PATH)?;
//!     let reader = BinlogReader::new(file)?;
//!
//!     for result in reader.read_events() {
//!         let (header, event) = result?;
//!         println!("{:#?}", header);
//!         println!("{:#?}", event);
//!     }
//!     Ok(())
//! }
//! ```

pub mod binlog_client;
pub mod binlog_events;
pub mod binlog_options;
pub mod binlog_raw_events;
pub mod binlog_reader;
pub mod errors;
pub mod events;
pub mod metadata;
pub mod providers;
pub mod replica_options;
pub mod ssl_mode;
pub mod starting_strategy;

mod commands;
mod configure;
mod connect;
mod constants;
mod extensions;
mod packet_channel;
mod responses;
