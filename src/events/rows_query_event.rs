use std::io::{Cursor, Read, Seek, SeekFrom};

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
    pub fn parse_mysql(cursor: &mut Cursor<&[u8]>) -> Self {
        cursor.seek(SeekFrom::Current(1)).unwrap();

        let mut query = String::new();
        cursor.read_to_string(&mut query).unwrap();

        Self { query }
    }

    /// Supports MariaDB 5.3+.
    pub fn parse_mariadb(cursor: &mut Cursor<&[u8]>) -> Self {
        let mut query = String::new();
        cursor.read_to_string(&mut query).unwrap();

        Self { query }
    }
}
