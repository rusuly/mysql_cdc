/// Represents the SSL strategy used to connect to the server.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SslMode {
    /// Establishes an unencrypted connection.
    DISABLED,

    /// Tries to establish an encrypted connection without verifying CA/Host. Falls back to an unencrypted connection.
    IF_AVAILABLE,

    /// Require an encrypted connection without verifying CA/Host.
    REQUIRE,

    /// Verify that the certificate belongs to the Certificate Authority.
    REQUIRE_VERIFY_CA,

    /// Verify that the certificate belongs to the Certificate Authority and matches Host.
    REQUIRE_VERIFY_FULL,
}
