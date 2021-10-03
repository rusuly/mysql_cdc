/// Checksum type used in a binlog file.
#[derive(Clone, Copy, Debug)]
pub enum ChecksumType {
    /// Checksum is disabled.
    NONE = 0,

    /// CRC32 checksum.
    CRC32 = 1,
}

impl ChecksumType {
    pub fn from_code(code: u8) -> Self {
        match code {
            0 => ChecksumType::NONE,
            1 => ChecksumType::CRC32,
            _ => panic!("The master checksum type is not supported: {}", code),
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "NONE" => ChecksumType::NONE,
            "CRC32" => ChecksumType::CRC32,
            _ => panic!("The master checksum type is not supported: {}", name),
        }
    }
}
