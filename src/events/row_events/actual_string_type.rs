use crate::constants::column_type::ColumnType;

/// Parses actual string type
/// See: https://bugs.mysql.com/bug.php?id=37426
/// See: https://github.com/mysql/mysql-server/blob/9c3a49ec84b521cb0b35383f119099b2eb25d4ff/sql/log_event.cc#L1988
pub fn get_actual_string_type(column_type: &mut u8, metadata: &mut u16) {
    // CHAR column type
    if *metadata < 256 {
        return;
    }

    // CHAR or ENUM or SET column types
    let byte0 = (*metadata >> 8) as u8;
    let byte1 = *metadata & 0xFF;

    if (byte0 & 0x30) != 0x30 {
        /* a long CHAR() field: see #37426 */
        *metadata = byte1 | (((byte0 as u16 & 0x30) ^ 0x30) << 4);
        *column_type = byte0 | 0x30;
    } else {
        if byte0 == ColumnType::Enum as u8 || byte0 == ColumnType::Set as u8 {
            *column_type = byte0;
        }
        *metadata = byte1;
    }
}

#[cfg(test)]
mod tests {
    use super::get_actual_string_type;
    use crate::constants::column_type::ColumnType;

    #[test]
    fn get_actual_string_type_char() {
        // char(200)
        let mut column_type = ColumnType::String as u8;
        let mut metadata: u16 = 52768;
        get_actual_string_type(&mut column_type, &mut metadata);

        assert_eq!(ColumnType::String as u8, column_type);
        assert_eq!(800 /* 200*Utf8Mb4 */, metadata);
    }

    #[test]
    fn get_actual_string_type_enum() {
        // enum('Low', 'Medium', 'High')
        let mut column_type = ColumnType::String as u8;
        let mut metadata: u16 = 63233;
        get_actual_string_type(&mut column_type, &mut metadata);

        assert_eq!(ColumnType::Enum as u8, column_type);
        assert_eq!(1, metadata);
    }

    #[test]
    fn get_actual_string_type_set() {
        // set('Green', 'Yellow', 'Red')
        let mut column_type = ColumnType::String as u8;
        let mut metadata: u16 = 63489;
        get_actual_string_type(&mut column_type, &mut metadata);

        assert_eq!(ColumnType::Set as u8, column_type);
        assert_eq!(1, metadata);
    }
}
