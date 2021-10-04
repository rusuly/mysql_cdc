pub enum MetadataType {
    SIGNEDNESS = 1,
    DEFAULT_CHARSET = 2,
    COLUMN_CHARSET = 3,
    COLUMN_NAME = 4,
    SET_STR_VALUE = 5,
    ENUM_STR_VALUE = 6,
    GEOMETRY_TYPE = 7,
    SIMPLE_PRIMARY_KEY = 8,
    PRIMARY_KEY_WITH_PREFIX = 9,
    ENUM_AND_SET_DEFAULT_CHARSET = 10,
    ENUM_AND_SET_COLUMN_CHARSET = 11,
    COLUMN_VISIBILITY = 12,
}

impl MetadataType {
    pub fn from_code(code: u8) -> Self {
        match code {
            1 => MetadataType::SIGNEDNESS,
            2 => MetadataType::DEFAULT_CHARSET,
            3 => MetadataType::COLUMN_CHARSET,
            4 => MetadataType::COLUMN_NAME,
            5 => MetadataType::SET_STR_VALUE,
            6 => MetadataType::ENUM_STR_VALUE,
            7 => MetadataType::GEOMETRY_TYPE,
            8 => MetadataType::SIMPLE_PRIMARY_KEY,
            9 => MetadataType::PRIMARY_KEY_WITH_PREFIX,
            10 => MetadataType::ENUM_AND_SET_DEFAULT_CHARSET,
            11 => MetadataType::ENUM_AND_SET_COLUMN_CHARSET,
            12 => MetadataType::COLUMN_VISIBILITY,
            _ => panic!("Table metadata type {} is not supported", code),
        }
    }
}
