use crate::events::format_description_event::FormatDescriptionEvent;
use crate::events::heartbeat_event::HeartbeatEvent;
use crate::events::intvar_event::IntVarEvent;
use crate::events::query_event::QueryEvent;
use crate::events::rotate_event::RotateEvent;
use crate::events::row_events::delete_rows_event::DeleteRowsEvent;
use crate::events::row_events::update_rows_event::UpdateRowsEvent;
use crate::events::row_events::write_rows_event::WriteRowsEvent;
use crate::events::rows_query_event::RowsQueryEvent;
use crate::events::table_map_event::TableMapEvent;
use crate::events::uservar_event::UserVarEvent;
use crate::events::xid_event::XidEvent;
use crate::providers::mariadb::events::gtid_event::GtidEvent as MariaDbGtidEvent;
use crate::providers::mariadb::events::gtid_list_event::GtidListEvent;
use crate::providers::mysql::events::gtid_event::GtidEvent as MySqlGtidEvent;
use crate::providers::mysql::events::prev_gtids_event::PreviousGtidsEvent;

/// Represents a binlog event.
#[derive(Debug)]
pub enum BinlogEvent {
    UnknownEvent,
    DeleteRowsEvent(DeleteRowsEvent),
    UpdateRowsEvent(UpdateRowsEvent),
    WriteRowsEvent(WriteRowsEvent),
    XidEvent(XidEvent),
    IntVarEvent(IntVarEvent),
    UserVarEvent(UserVarEvent),
    QueryEvent(QueryEvent),
    TableMapEvent(TableMapEvent),
    RotateEvent(RotateEvent),
    RowsQueryEvent(RowsQueryEvent),
    HeartbeatEvent(HeartbeatEvent),
    FormatDescriptionEvent(FormatDescriptionEvent),
    // Provider specific events
    MySqlGtidEvent(MySqlGtidEvent),
    MySqlPrevGtidsEvent(PreviousGtidsEvent),
    MariaDbGtidEvent(MariaDbGtidEvent),
    MariaDbGtidListEvent(GtidListEvent),
}
