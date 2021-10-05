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
use crate::events::xid_event::XidEvent;

/// Represents a binlog event.
#[derive(Debug)]
pub enum BinlogEvent {
    UnknownEvent,
    DeleteRowsEvent(DeleteRowsEvent),
    UpdateRowsEvent(UpdateRowsEvent),
    WriteRowsEvent(WriteRowsEvent),
    XidEvent(XidEvent),
    IntVarEvent(IntVarEvent),
    QueryEvent(QueryEvent),
    TableMapEvent(TableMapEvent),
    RotateEvent(RotateEvent),
    RowsQueryEvent(RowsQueryEvent),
    HeartbeatEvent(HeartbeatEvent),
    FormatDescriptionEvent(FormatDescriptionEvent),
}
