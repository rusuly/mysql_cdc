use crate::events::row_events::mysql_value::MySqlValue;

/// Represents an inserted or deleted row in row based replication.
#[derive(Debug)]
pub struct RowData {
    /// Column values of the changed row.
    pub cells: Vec<Option<MySqlValue>>,
}

impl RowData {
    pub fn new(cells: Vec<Option<MySqlValue>>) -> Self {
        Self { cells }
    }
}

/// Represents an updated row in row based replication.
#[derive(Debug)]
pub struct UpdateRowData {
    /// Row state before it was updated.
    pub before_update: RowData,

    /// Actual row state after update.
    pub after_update: RowData,
}

impl UpdateRowData {
    pub fn new(before_update: RowData, after_update: RowData) -> Self {
        Self {
            before_update,
            after_update,
        }
    }
}
