use crate::events::format_description_event::FormatDescriptionEvent;
use crate::events::rotate_event::RotateEvent;

/// Represents a binlog event.
#[derive(Debug)]
pub enum BinlogEvent {
    UnknownEvent,
    RotateEvent(RotateEvent),
    FormatDescriptionEvent(FormatDescriptionEvent),
}
