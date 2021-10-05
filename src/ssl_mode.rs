/// Represents the SSL strategy used to connect to the server.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SslMode {
    /// Establishes an unencrypted connection.
    Disabled = 0,

    /// Tries to establish an encrypted connection without verifying CA/Host. Falls back to an unencrypted connection.
    IfAvailable = 1,

    /// Require an encrypted connection without verifying CA/Host.
    Require = 2,

    /// Verify that the certificate belongs to the Certificate Authority.
    RequireVerifyCa = 3,

    /// Verify that the certificate belongs to the Certificate Authority and matches Host.
    RequireVerifyFull = 4,
}
