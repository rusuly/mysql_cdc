use crate::errors::Error;

pub enum MetadataType {
    Signedness = 1,
    DefaultCharset = 2,
    ColumnCharset = 3,
    ColumnName = 4,
    SetStrValue = 5,
    EnumStrValue = 6,
    GeometryType = 7,
    SimplePrimaryKey = 8,
    PrimaryKeyWithPrefix = 9,
    EnumAndSetDefaultCharset = 10,
    EnumAndSetColumnCharset = 11,
    ColumnVisibility = 12,
}

impl MetadataType {
    pub fn from_code(code: u8) -> Result<Self, Error> {
        let value = match code {
            1 => MetadataType::Signedness,
            2 => MetadataType::DefaultCharset,
            3 => MetadataType::ColumnCharset,
            4 => MetadataType::ColumnName,
            5 => MetadataType::SetStrValue,
            6 => MetadataType::EnumStrValue,
            7 => MetadataType::GeometryType,
            8 => MetadataType::SimplePrimaryKey,
            9 => MetadataType::PrimaryKeyWithPrefix,
            10 => MetadataType::EnumAndSetDefaultCharset,
            11 => MetadataType::EnumAndSetColumnCharset,
            12 => MetadataType::ColumnVisibility,
            _ => {
                return Err(Error::String(
                    format!("Table metadata type {} is not supported", code).to_string(),
                ))
            }
        };
        Ok(value)
    }
}
