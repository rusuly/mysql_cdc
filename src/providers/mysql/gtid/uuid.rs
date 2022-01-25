use std::fmt;

use crate::errors::Error;

/// Represents Uuid with little-endian bytes order unlike big-endian Guid.
#[derive(Clone, Debug)]
pub struct Uuid {
    pub data: [u8; 16],
    pub uuid: String,
}

impl Uuid {
    pub fn new(data: [u8; 16]) -> Self {
        let mut uuid = hex::encode(data);
        uuid.insert(20, '-');
        uuid.insert(16, '-');
        uuid.insert(12, '-');
        uuid.insert(8, '-');
        Self { data, uuid }
    }

    /// Parses Uuid from string representation.
    pub fn parse(uuid: String) -> Result<Self, Error> {
        let hex = uuid.replace("-", "");
        let vec = hex::decode(hex)?;

        let mut data = [0u8; 16];
        (0..16).for_each(|i| data[i] = vec[i]);

        Ok(Self { data, uuid })
    }
}

impl fmt::Display for Uuid {
    /// Returns string representation of the UUID.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uuid)
    }
}
