use serde::de::Deserialize;
use serde::de::Unexpected;
use serde::de::Visitor;
use serde::ser::Serialize;
use serde::Deserializer;
use serde::Serializer;
use std::fmt;

use crate::errors::Error;
use serde::de::Error as SeError;
use std::string::String;

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

impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.uuid.as_str())
    }
}
struct StringVisitor;
impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: SeError,
    {
        Ok(v.to_owned())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: SeError,
    {
        Ok(v)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: SeError,
    {
        match std::str::from_utf8(v) {
            Ok(s) => Ok(s.to_owned()),
            Err(_) => Err(SeError::invalid_value(Unexpected::Bytes(v), &self)),
        }
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: SeError,
    {
        match String::from_utf8(v) {
            Ok(s) => Ok(s),
            Err(e) => Err(SeError::invalid_value(
                Unexpected::Bytes(&e.into_bytes()),
                &self,
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let uuid = deserializer.deserialize_string(StringVisitor)?;
        Ok(Uuid::parse(uuid).unwrap())
    }
}

impl fmt::Display for Uuid {
    /// Returns string representation of the UUID.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uuid)
    }
}
