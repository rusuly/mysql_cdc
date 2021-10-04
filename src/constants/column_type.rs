/// MySql column types.
/// See <a href="https://mariadb.com/kb/en/library/resultset/#column-definition-packet">MariaDB docs</a>
/// See <a href="https://dev.mysql.com/doc/internals/en/com-query-response.html#column-type">MySQL docs</a>
pub enum ColumnType {
    /// DECIMAL
    DECIMAL = 0,

    /// TINY
    TINY = 1,

    /// SHORT
    SHORT = 2,

    /// LONG
    LONG = 3,

    /// FLOAT
    FLOAT = 4,

    /// DOUBLE
    DOUBLE = 5,

    /// NULL
    NULL = 6,

    /// TIMESTAMP
    TIMESTAMP = 7,

    /// LONGLONG
    LONGLONG = 8,

    /// INT24
    INT24 = 9,

    /// DATE
    DATE = 10,

    /// TIME
    TIME = 11,

    /// DATETIME
    DATETIME = 12,

    /// YEAR
    YEAR = 13,

    /// NEWDATE
    NEWDATE = 14,

    /// VARCHAR
    VARCHAR = 15,

    /// BIT
    BIT = 16,

    /// TIMESTAMP2
    TIMESTAMP2 = 17,

    /// DATETIME2
    DATETIME2 = 18,

    /// TIME2
    TIME2 = 19,

    /// JSON is MySQL 5.7.8+ type. Not supported in MariaDB.
    JSON = 245,

    /// NEWDECIMAL
    NEWDECIMAL = 246,

    /// ENUM
    ENUM = 247,

    /// SET
    SET = 248,

    /// TINY_BLOB
    TINY_BLOB = 249,

    /// MEDIUM_BLOB
    MEDIUM_BLOB = 250,

    /// LONG_BLOB
    LONG_BLOB = 251,

    /// BLOB
    BLOB = 252,

    /// VAR_STRING
    VAR_STRING = 253,

    /// STRING
    STRING = 254,

    /// GEOMETRY
    GEOMETRY = 255,
}

impl ColumnType {
    pub fn from_code(code: u8) -> Self {
        match code {
            0 => ColumnType::DECIMAL,
            1 => ColumnType::TINY,
            2 => ColumnType::SHORT,
            3 => ColumnType::LONG,
            4 => ColumnType::FLOAT,
            5 => ColumnType::DOUBLE,
            6 => ColumnType::NULL,
            7 => ColumnType::TIMESTAMP,
            8 => ColumnType::LONGLONG,
            9 => ColumnType::INT24,
            10 => ColumnType::DATE,
            11 => ColumnType::TIME,
            12 => ColumnType::DATETIME,
            13 => ColumnType::YEAR,
            14 => ColumnType::NEWDATE,
            15 => ColumnType::VARCHAR,
            16 => ColumnType::BIT,
            17 => ColumnType::TIMESTAMP2,
            18 => ColumnType::DATETIME2,
            19 => ColumnType::TIME2,
            245 => ColumnType::JSON,
            246 => ColumnType::NEWDECIMAL,
            247 => ColumnType::ENUM,
            248 => ColumnType::SET,
            249 => ColumnType::TINY_BLOB,
            250 => ColumnType::MEDIUM_BLOB,
            251 => ColumnType::LONG_BLOB,
            252 => ColumnType::BLOB,
            253 => ColumnType::VAR_STRING,
            254 => ColumnType::STRING,
            255 => ColumnType::GEOMETRY,
            _ => panic!("Unknown column type {}", code),
        }
    }
}
