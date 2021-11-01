use std::io::{Cursor, Read, Seek, SeekFrom};

use crate::errors::Error;

/// Represents query that caused row events.
/// See <a href="https://dev.mysql.com/doc/internals/en/rows-query-event.html">MySQL docs</a>
/// See <a href="https://mariadb.com/kb/en/annotate_rows_event/">MariaDB docs</a>
#[derive(Debug)]
pub struct RowsQueryEvent {
    /// Gets SQL statement
    pub query: String,
}

impl RowsQueryEvent {
    /// Supports MySQL 5.6+.
    pub fn parse_mysql(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        cursor.seek(SeekFrom::Current(1))?;

        let mut query = String::new();
        cursor.read_to_string(&mut query)?;

        Ok(Self { query })
    }

    /// Supports MariaDB 5.3+.
    pub fn parse_mariadb(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let mut query = String::new();
        cursor.read_to_string(&mut query)?;

        Ok(Self { query })
    }
}
