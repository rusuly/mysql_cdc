/// Binlog event types.
/// See <a href="https://mariadb.com/kb/en/library/2-binlog-event-header/">event header docs</a>
/// See <a href="https://dev.mysql.com/doc/internals/en/binlog-event-type.html">list of event types</a>
/// See <a href="https://dev.mysql.com/doc/internals/en/rows-event.html#write-rows-eventv2">rows event docs</a>
pub enum EventType {
    Unknown,

    /// Identifies <see cref="QueryEvent"/>.
    QueryEvent = 2,

    /// Identifies StopEvent.
    StopEvent = 3,

    /// Identifies <see cref="RotateEvent"/>.
    RotateEvent = 4,

    /// Identifies <see cref="IntVarEvent"/>.
    IntvarEvent = 5,

    /// Identifies RandEvent.
    RandEvent = 13,

    /// Identifies UserVarEvent.
    UserVarEvent = 14,

    /// Identifies <see cref="FormatDescriptionEvent"/>.
    FormatDescriptionEvent = 15,

    /// Identifies <see cref="XidEvent"/>.
    XidEvent = 16,

    /// Identifies <see cref="TableMapEvent"/>.
    TableMapEvent = 19,

    /// Row events
    /// Identifies <see cref="WriteRowsEvent"/> in MariaDB and MySQL from 5.1.15 to 5.6.
    WriteRowsEventV1 = 23,

    /// Identifies <see cref="UpdateRowsEvent"/> in MariaDB and MySQL from 5.1.15 to 5.6.
    UpdateRowsEventV1 = 24,

    /// Identifies <see cref="DeleteRowsEvent"/> in MariaDB and MySQL from 5.1.15 to 5.6.
    DeleteRowsEventV1 = 25,

    /// Identifies <see cref="HeartbeatEvent"/>.
    HeartbeatEvent = 27,

    /// MySQL specific events
    /// Identifies <see cref="RowsQueryEvent"/> in MySQL from 5.6 to 8.0.
    MySqlRowsQueryEvent = 29,

    /// Identifies <see cref="WriteRowsEvent"/> in MySQL from 5.6 to 8.0.
    MySqlWriteRowsEventV2 = 30,

    /// Identifies <see cref="UpdateRowsEvent"/> in MySQL from 5.6 to 8.0.
    MySqlUpdateRowsEventV2 = 31,

    /// Identifies <see cref="DeleteRowsEvent"/> in MySQL from 5.6 to 8.0.
    MySqlDeleteRowsEventV2 = 32,

    /// Identifies <see cref="GtidEvent"/> in MySQL from 5.6 to 8.0.
    MySqlGtidEvent = 33,

    /// Identifies <see cref="PreviousGtidsEvent"/> in MySQL from 5.6 to 8.0.
    MySqlPreviousGtidsEvent = 35,

    /// Identifies <see cref="XaPrepareEvent"/> in MySQL from 5.6 to 8.0.
    MySqlXaPrepare = 38,

    /// MariaDB specific events
    /// Identifies <see cref="RowsQueryEvent"/> in MariaDB.
    MariaDbAnnotateRowsEvent = 160,

    /// Identifies binlog checkpoint event in MariaDB.
    MariaDbBinlogCheckpointEvent = 161,

    /// Identifies <see cref="GtidEvent"/> in MariaDB.
    MariaDbGtidEvent = 162,

    /// Identifies <see cref="GtidListEvent"/> in MariaDB.
    MariaDbGtidListEvent = 163,

    /// Identifies encryption start event in MariaDB.
    MariaDbStartEncryptionEvent = 164,
}

impl EventType {
    pub fn from_code(code: u8) -> Self {
        match code {
            2 => EventType::QueryEvent,
            3 => EventType::StopEvent,
            4 => EventType::RotateEvent,
            5 => EventType::IntvarEvent,
            13 => EventType::RandEvent,
            14 => EventType::UserVarEvent,
            15 => EventType::FormatDescriptionEvent,
            16 => EventType::XidEvent,
            19 => EventType::TableMapEvent,
            23 => EventType::WriteRowsEventV1,
            24 => EventType::UpdateRowsEventV1,
            25 => EventType::DeleteRowsEventV1,
            27 => EventType::HeartbeatEvent,
            29 => EventType::MySqlRowsQueryEvent,
            30 => EventType::MySqlWriteRowsEventV2,
            31 => EventType::MySqlUpdateRowsEventV2,
            32 => EventType::MySqlDeleteRowsEventV2,
            33 => EventType::MySqlGtidEvent,
            35 => EventType::MySqlPreviousGtidsEvent,
            38 => EventType::MySqlXaPrepare,
            160 => EventType::MariaDbAnnotateRowsEvent,
            161 => EventType::MariaDbBinlogCheckpointEvent,
            162 => EventType::MariaDbGtidEvent,
            163 => EventType::MariaDbGtidListEvent,
            164 => EventType::MariaDbStartEncryptionEvent,
            _ => EventType::Unknown,
        }
    }
}
