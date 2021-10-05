/// Checksum type used in a binlog file.
#[derive(Clone, Copy, Debug)]
pub enum ChecksumType {
    /// Checksum is disabled.
    None = 0,

    /// CRC32 checksum.
    Crc32 = 1,
}

impl ChecksumType {
    pub fn from_code(code: u8) -> Self {
        match code {
            0 => ChecksumType::None,
            1 => ChecksumType::Crc32,
            _ => panic!("The master checksum type is not supported: {}", code),
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "NONE" => ChecksumType::None,
            "CRC32" => ChecksumType::Crc32,
            _ => panic!("The master checksum type is not supported: {}", name),
        }
    }
}
