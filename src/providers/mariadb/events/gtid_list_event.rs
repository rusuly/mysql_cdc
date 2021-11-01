use crate::providers::mariadb::gtid::gtid_list::GtidList;
use crate::{errors::Error, providers::mariadb::gtid::gtid::Gtid};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

/// Shows current replication state with list of last gtid for each replication domain.
/// <a href="https://mariadb.com/kb/en/gtid_list_event/">See more</a>
#[derive(Debug)]
pub struct GtidListEvent {
    /// Gets a list of Gtid that represents current replication state
    pub gtid_list: GtidList,
}

impl GtidListEvent {
    /// Parses events in MariaDB.
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let gtid_list_len = cursor.read_u32::<LittleEndian>()?;

        let mut gtid_list = GtidList::new();
        for _i in 0..gtid_list_len {
            let domain_id = cursor.read_u32::<LittleEndian>()?;
            let server_id = cursor.read_u32::<LittleEndian>()?;
            let sequence = cursor.read_u64::<LittleEndian>()?;

            let gtid = Gtid::new(domain_id, server_id, sequence);
            gtid_list.gtids.push(gtid);
        }

        Ok(Self { gtid_list })
    }
}
