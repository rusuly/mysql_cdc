use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom};

/// Represents sql statement in binary log.
/// <a href="https://mariadb.com/kb/en/library/query_event/">See more</a>
#[derive(Debug)]
pub struct QueryEvent {
    /// Gets id of the thread that issued the statement.
    pub thread_id: u32,

    /// Gets the execution time of the statement in seconds.
    pub duration: u32,

    /// Gets the error code of the executed statement.
    pub error_code: u16,

    /// Gets status variables.
    pub status_variables: Vec<u8>,

    /// Gets the default database name.
    pub database_name: String,

    /// Gets the SQL statement.
    pub sql_statement: String,
}

impl QueryEvent {
    /// Supports all versions of MariaDB and MySQL.
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Self {
        let thread_id = cursor.read_u32::<LittleEndian>().unwrap();
        let duration = cursor.read_u32::<LittleEndian>().unwrap();

        // DatabaseName length is null terminated
        let database_name_length = cursor.read_u8().unwrap();

        let error_code = cursor.read_u16::<LittleEndian>().unwrap();
        let status_variable_length = cursor.read_u16::<LittleEndian>().unwrap();

        let mut status_variables: Vec<u8> = vec![0; status_variable_length as usize];
        cursor
            .read_exact(&mut status_variables[0..status_variable_length as usize])
            .unwrap();

        let mut database_name: Vec<u8> = vec![0; database_name_length as usize];
        cursor
            .read_exact(&mut database_name[0..database_name_length as usize])
            .unwrap();

        let database_name = String::from_utf8(database_name).unwrap();

        cursor.seek(SeekFrom::Current(1)).unwrap();

        let mut sql_statement = String::new();
        cursor.read_to_string(&mut sql_statement).unwrap();

        Self {
            thread_id,
            duration,
            error_code,
            status_variables,
            database_name,
            sql_statement,
        }
    }
}
