/// Used by default in MariaDB and MySQL 5.7 Server and prior.
pub const MY_SQL_NATIVE_PASSWORD: &str = "mysql_native_password";

/// Used by default in MySQL Server 8.0.
pub const CACHING_SHA2_PASSWORD: &str = "caching_sha2_password";

pub enum AuthPlugin {
    MySqlNativePassword,
    CachingSha2Password,
}
