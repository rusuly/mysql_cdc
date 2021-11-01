use crate::errors::Error;
use crate::providers::mariadb::gtid::gtid::Gtid;
use std::collections::HashSet;
use std::fmt;

/// Represents GtidList from MariaDB.
#[derive(Debug)]
pub struct GtidList {
    /// Gets a list of Gtids per each domain.
    pub gtids: Vec<Gtid>,
}

impl GtidList {
    pub fn new() -> Self {
        Self { gtids: Vec::new() }
    }

    /// Parses from string representation.
    pub fn parse(value: &str) -> Result<Self, Error> {
        if value.is_empty() {
            return Ok(GtidList::new());
        }

        let value = value.replace("\n", "");
        let gtid_list = value.split(',').map(|x| x.trim()).collect::<Vec<&str>>();

        let mut domain_map = HashSet::new();
        let mut gtids = Vec::new();

        for gtid in gtid_list {
            let components = gtid.split('-').collect::<Vec<&str>>();
            let domain_id: u32 = components[0].parse()?;
            let server_id: u32 = components[1].parse()?;
            let sequence: u64 = components[2].parse()?;

            if domain_map.contains(&domain_id) {
                return Err(Error::String(format!(
                    "GtidList must consist of unique domain ids"
                )));
            } else {
                domain_map.insert(domain_id);
            }

            gtids.push(Gtid::new(domain_id, server_id, sequence));
        }

        Ok(Self { gtids })
    }

    /// Adds a gtid value to the GtidList.
    pub fn add_gtid(&mut self, gtid: Gtid) -> bool {
        for i in 0..self.gtids.len() {
            if self.gtids[i].domain_id == gtid.domain_id {
                self.gtids[i] = gtid;
                return false;
            }
        }
        self.gtids.push(gtid);
        true
    }
}

impl fmt::Display for GtidList {
    /// Returns string representation of the GtidList.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = self
            .gtids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        write!(f, "{}", str)
    }
}
