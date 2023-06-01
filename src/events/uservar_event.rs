use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

use crate::{errors::Error, extensions::read_string};

/// A USER_VAR_EVENT is written every time a statement uses a user defined variable.
/// <a href="https://mariadb.com/kb/en/user_var_event/">See more</a>
#[derive(Debug)]
pub struct UserVarEvent {
    /// User variable name
    pub name: String,

    /// User variable value
    pub value: Option<VariableValue>,
}

/// User variable value
#[derive(Debug)]
pub struct VariableValue {
    /// Variable type
    pub var_type: u8,

    /// Collation number
    pub collation: u32,

    /// User variable value
    pub value: String,

    /// flags
    pub flags: u8,
}

impl UserVarEvent {
    /// Supports all versions of MariaDB and MySQL.
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let name_len = cursor.read_u32::<LittleEndian>()?;
        let name = read_string(cursor, name_len as usize)?;

        let is_null = cursor.read_u8()? != 0; // 0 indicates there is a value;
        if is_null {
            return Ok(Self { name, value: None });
        }

        let var_type = cursor.read_u8()?;
        let collation = cursor.read_u32::<LittleEndian>()?;

        let value_len = cursor.read_u32::<LittleEndian>()?;
        let value = read_string(cursor, value_len as usize)?;

        let flags = cursor.read_u8()?;

        Ok(Self {
            name,
            value: Some(VariableValue {
                var_type,
                collation,
                value,
                flags,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::events::uservar_event::UserVarEvent;

    #[test]
    fn parse_user_var_event() {
        let payload: Vec<u8> = vec![
            0x03, 0x00, 0x00, 0x00, 0x66, 0x6f, 0x6f, 0x00, 0x00, 0x21, 0x00, 0x00, 0x00, 0x03,
            0x00, 0x00, 0x00, 0x62, 0x61, 0x72, 0x6b, 0x3d, 0xd9, 0x7d,
        ];
        let mut cursor = Cursor::new(payload.as_slice());

        let event = UserVarEvent::parse(&mut cursor).unwrap();
        assert_eq!(String::from("foo"), event.name);
        assert_eq!(false, event.value.is_none());

        let variable = event.value.unwrap();
        assert_eq!(0, variable.var_type);
        assert_eq!(33, variable.collation);
        assert_eq!(String::from("bar"), variable.value);
    }
}
