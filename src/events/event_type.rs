/// Binlog event types.
/// See <a href="https://mariadb.com/kb/en/library/2-binlog-event-header/">event header docs</a>
/// See <a href="https://dev.mysql.com/doc/internals/en/binlog-event-type.html">list of event types</a>
/// See <a href="https://dev.mysql.com/doc/internals/en/rows-event.html#write-rows-eventv2">rows event docs</a>
pub enum EventType {
    Unknown,

    /// Identifies <see cref="QueryEvent"/>.
    QUERY_EVENT = 2,

    /// Identifies StopEvent.
    STOP_EVENT = 3,

    /// Identifies <see cref="RotateEvent"/>.
    ROTATE_EVENT = 4,

    /// Identifies <see cref="IntVarEvent"/>.
    INTVAR_EVENT = 5,

    /// Identifies RandEvent.
    RAND_EVENT = 13,

    /// Identifies UserVarEvent.
    USER_VAR_EVENT = 14,

    /// Identifies <see cref="FormatDescriptionEvent"/>.
    FORMAT_DESCRIPTION_EVENT = 15,

    /// Identifies <see cref="XidEvent"/>.
    XID_EVENT = 16,

    /// Identifies <see cref="TableMapEvent"/>.
    TABLE_MAP_EVENT = 19,

    /// Row events
    /// Identifies <see cref="WriteRowsEvent"/> in MariaDB and MySQL from 5.1.15 to 5.6.
    WRITE_ROWS_EVENT_V1 = 23,

    /// Identifies <see cref="UpdateRowsEvent"/> in MariaDB and MySQL from 5.1.15 to 5.6.
    UPDATE_ROWS_EVENT_V1 = 24,

    /// Identifies <see cref="DeleteRowsEvent"/> in MariaDB and MySQL from 5.1.15 to 5.6.
    DELETE_ROWS_EVENT_V1 = 25,

    /// Identifies <see cref="HeartbeatEvent"/>.
    HEARTBEAT_EVENT = 27,

    /// MySQL specific events
    /// Identifies <see cref="RowsQueryEvent"/> in MySQL from 5.6 to 8.0.
    MYSQL_ROWS_QUERY_EVENT = 29,

    /// Identifies <see cref="WriteRowsEvent"/> in MySQL from 5.6 to 8.0.
    MYSQL_WRITE_ROWS_EVENT_V2 = 30,

    /// Identifies <see cref="UpdateRowsEvent"/> in MySQL from 5.6 to 8.0.
    MYSQL_UPDATE_ROWS_EVENT_V2 = 31,

    /// Identifies <see cref="DeleteRowsEvent"/> in MySQL from 5.6 to 8.0.
    MYSQL_DELETE_ROWS_EVENT_V2 = 32,

    /// Identifies <see cref="GtidEvent"/> in MySQL from 5.6 to 8.0.
    MYSQL_GTID_EVENT = 33,

    /// Identifies <see cref="PreviousGtidsEvent"/> in MySQL from 5.6 to 8.0.
    MYSQL_PREVIOUS_GTIDS_EVENT = 35,

    /// Identifies <see cref="XaPrepareEvent"/> in MySQL from 5.6 to 8.0.
    MYSQL_XA_PREPARE = 38,

    /// MariaDB specific events
    /// Identifies <see cref="RowsQueryEvent"/> in MariaDB.
    MARIADB_ANNOTATE_ROWS_EVENT = 160,

    /// Identifies binlog checkpoint event in MariaDB.
    MARIADB_BINLOG_CHECKPOINT_EVENT = 161,

    /// Identifies <see cref="GtidEvent"/> in MariaDB.
    MARIADB_GTID_EVENT = 162,

    /// Identifies <see cref="GtidListEvent"/> in MariaDB.
    MARIADB_GTID_LIST_EVENT = 163,

    /// Identifies encryption start event in MariaDB.
    MARIADB_START_ENCRYPTION_EVENT = 164,
}

impl EventType {
    pub fn from_code(code: u8) -> Self {
        match code {
            2 => EventType::QUERY_EVENT,
            3 => EventType::STOP_EVENT,
            4 => EventType::ROTATE_EVENT,
            5 => EventType::INTVAR_EVENT,
            13 => EventType::RAND_EVENT,
            14 => EventType::USER_VAR_EVENT,
            15 => EventType::FORMAT_DESCRIPTION_EVENT,
            16 => EventType::XID_EVENT,
            19 => EventType::TABLE_MAP_EVENT,
            23 => EventType::WRITE_ROWS_EVENT_V1,
            24 => EventType::UPDATE_ROWS_EVENT_V1,
            25 => EventType::DELETE_ROWS_EVENT_V1,
            27 => EventType::HEARTBEAT_EVENT,
            29 => EventType::MYSQL_ROWS_QUERY_EVENT,
            30 => EventType::MYSQL_WRITE_ROWS_EVENT_V2,
            31 => EventType::MYSQL_UPDATE_ROWS_EVENT_V2,
            32 => EventType::MYSQL_DELETE_ROWS_EVENT_V2,
            33 => EventType::MYSQL_GTID_EVENT,
            35 => EventType::MYSQL_PREVIOUS_GTIDS_EVENT,
            38 => EventType::MYSQL_XA_PREPARE,
            160 => EventType::MARIADB_ANNOTATE_ROWS_EVENT,
            161 => EventType::MARIADB_BINLOG_CHECKPOINT_EVENT,
            162 => EventType::MARIADB_GTID_EVENT,
            163 => EventType::MARIADB_GTID_LIST_EVENT,
            164 => EventType::MARIADB_START_ENCRYPTION_EVENT,
            _ => EventType::Unknown,
        }
    }
}
