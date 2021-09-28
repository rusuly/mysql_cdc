mod commands;
mod constants;

pub mod binlog_reader;
pub mod events;
pub mod options;
pub mod ssl_mode;

use crate::options::ReplicaOptions;
use crate::ssl_mode::SslMode;
use std::net::TcpStream;

pub fn replicate(options: &ReplicaOptions) {
    if options.ssl_mode == SslMode::REQUIRE_VERIFY_CA
        || options.ssl_mode == SslMode::REQUIRE_VERIFY_FULL
    {
        unimplemented!(
            "{:?} and {:?} ssl modes are not supported",
            SslMode::REQUIRE_VERIFY_CA,
            SslMode::REQUIRE_VERIFY_FULL
        );
    }
}
