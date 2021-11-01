use crate::errors::Error;

/// Checksum type used in a binlog file.
#[derive(Clone, Copy, Debug)]
pub enum ChecksumType {
    /// Checksum is disabled.
    None = 0,

    /// CRC32 checksum.
    Crc32 = 1,
}

impl ChecksumType {
    pub fn from_code(code: u8) -> Result<Self, Error> {
        match code {
            0 => Ok(ChecksumType::None),
            1 => Ok(ChecksumType::Crc32),
            _ => Err(Error::String(
                format!("The master checksum type is not supported: {}", code).to_string(),
            )),
        }
    }

    pub fn from_name(name: &str) -> Result<Self, Error> {
        match name {
            "NONE" => Ok(ChecksumType::None),
            "CRC32" => Ok(ChecksumType::Crc32),
            _ => Err(Error::String(
                format!("The master checksum type is not supported: {}", name).to_string(),
            )),
        }
    }
}
