/// MySql column types.
/// See <a href="https://mariadb.com/kb/en/library/resultset/#column-definition-packet">MariaDB docs</a>
/// See <a href="https://dev.mysql.com/doc/internals/en/com-query-response.html#column-type">MySQL docs</a>
pub enum ColumnType {
    /// DECIMAL
    DECIMAL = 0,

    /// TINY
    TINY,

    /// SHORT
    SHORT,

    /// LONG
    LONG,

    /// FLOAT
    FLOAT,

    /// DOUBLE
    DOUBLE,

    /// NULL
    NULL,

    /// TIMESTAMP
    TIMESTAMP,

    /// LONGLONG
    LONGLONG,

    /// INT24
    INT24,

    /// DATE
    DATE,

    /// TIME
    TIME,

    /// DATETIME
    DATETIME,

    /// YEAR
    YEAR,

    /// NEWDATE
    NEWDATE,

    /// VARCHAR
    VARCHAR,

    /// BIT
    BIT,

    /// TIMESTAMP2
    TIMESTAMP2,

    /// DATETIME2
    DATETIME2,

    /// TIME2
    TIME2,

    /// JSON is MySQL 5.7.8+ type. Not supported in MariaDB.
    JSON = 245,

    /// NEWDECIMAL
    NEWDECIMAL,

    /// ENUM
    ENUM,

    /// SET
    SET,

    /// TINY_BLOB
    TINY_BLOB,

    /// MEDIUM_BLOB
    MEDIUM_BLOB,

    /// LONG_BLOB
    LONG_BLOB,

    /// BLOB
    BLOB,

    /// VAR_STRING
    VAR_STRING,

    /// STRING
    STRING,

    /// GEOMETRY
    GEOMETRY,
}
