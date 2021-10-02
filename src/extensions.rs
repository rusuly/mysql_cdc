use crate::constants::NULL_TERMINATOR;
use std::io::{BufRead, Cursor, Read};

pub fn read_null_term_string(cursor: &mut Cursor<&[u8]>) -> String {
    let mut vec = Vec::new();
    cursor.read_until(NULL_TERMINATOR, &mut vec).unwrap();
    vec.pop();
    String::from_utf8(vec).unwrap()
}

pub fn read_string(cursor: &mut Cursor<&[u8]>, size: usize) -> String {
    let mut vec = vec![0; size];
    cursor.read_exact(&mut vec).unwrap();
    String::from_utf8(vec).unwrap()
}
