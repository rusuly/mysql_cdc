use crate::errors::Error;

/// MySql column types.
/// See <a href="https://mariadb.com/kb/en/library/resultset/#column-definition-packet">MariaDB docs</a>
/// See <a href="https://dev.mysql.com/doc/internals/en/com-query-response.html#column-type">MySQL docs</a>
#[derive(PartialEq, Debug)]
pub enum ColumnType {
    /// DECIMAL
    Decimal = 0,

    /// TINY
    Tiny = 1,

    /// SHORT
    Short = 2,

    /// LONG
    Long = 3,

    /// FLOAT
    Float = 4,

    /// DOUBLE
    Double = 5,

    /// NULL
    Null = 6,

    /// TIMESTAMP
    TimeStamp = 7,

    /// LONGLONG
    LongLong = 8,

    /// INT24
    Int24 = 9,

    /// DATE
    Date = 10,

    /// TIME
    Time = 11,

    /// DATETIME
    DateTime = 12,

    /// YEAR
    Year = 13,

    /// NEWDATE
    NewDate = 14,

    /// VARCHAR
    VarChar = 15,

    /// BIT
    Bit = 16,

    /// TIMESTAMP2
    TimeStamp2 = 17,

    /// DATETIME2
    DateTime2 = 18,

    /// TIME2
    Time2 = 19,

    /// JSON is MySQL 5.7.8+ type. Not supported in MariaDB.
    Json = 245,

    /// NEWDECIMAL
    NewDecimal = 246,

    /// ENUM
    Enum = 247,

    /// SET
    Set = 248,

    /// TINY_BLOB
    TinyBlob = 249,

    /// MEDIUM_BLOB
    MediumBlob = 250,

    /// LONG_BLOB
    LongBlob = 251,

    /// BLOB
    Blob = 252,

    /// VAR_STRING
    VarString = 253,

    /// STRING
    String = 254,

    /// GEOMETRY
    Geometry = 255,
}

impl ColumnType {
    pub fn from_code(code: u8) -> Result<Self, Error> {
        let value = match code {
            0 => ColumnType::Decimal,
            1 => ColumnType::Tiny,
            2 => ColumnType::Short,
            3 => ColumnType::Long,
            4 => ColumnType::Float,
            5 => ColumnType::Double,
            6 => ColumnType::Null,
            7 => ColumnType::TimeStamp,
            8 => ColumnType::LongLong,
            9 => ColumnType::Int24,
            10 => ColumnType::Date,
            11 => ColumnType::Time,
            12 => ColumnType::DateTime,
            13 => ColumnType::Year,
            14 => ColumnType::NewDate,
            15 => ColumnType::VarChar,
            16 => ColumnType::Bit,
            17 => ColumnType::TimeStamp2,
            18 => ColumnType::DateTime2,
            19 => ColumnType::Time2,
            245 => ColumnType::Json,
            246 => ColumnType::NewDecimal,
            247 => ColumnType::Enum,
            248 => ColumnType::Set,
            249 => ColumnType::TinyBlob,
            250 => ColumnType::MediumBlob,
            251 => ColumnType::LongBlob,
            252 => ColumnType::Blob,
            253 => ColumnType::VarString,
            254 => ColumnType::String,
            255 => ColumnType::Geometry,
            _ => return Err(Error::String(format!("Unknown column type {}", code))),
        };
        Ok(value)
    }
}
