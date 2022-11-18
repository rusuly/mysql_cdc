use crate::errors::Error;
use crate::providers::mariadb::gtid::gtid::Gtid;
use std::collections::HashSet;
use std::fmt;

/// Represents GtidList from MariaDB.
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use super::GtidList;
    use crate::providers::mariadb::gtid::gtid::Gtid;

    #[test]
    #[should_panic(expected = "GtidList must consist of unique domain ids")]
    fn parse_not_unique_domains() {
        GtidList::parse("1-1-270, 1-1-271").unwrap();
    }

    #[test]
    fn parse_empty_string_returns_empty_gtid_list() {
        let empty = String::from("");
        let gtid_list = GtidList::parse(&empty).unwrap();

        assert_eq!(0, gtid_list.gtids.len());
        assert_eq!(empty, gtid_list.to_string());
    }

    #[test]
    fn parse_gtid_lists_returns_multiple_results() {
        let gtid_list1 = GtidList::parse("0-1-270").unwrap();
        let gtid_list2 = GtidList::parse("1-2-120,2-3-130").unwrap();
        let gtid_list3 = GtidList::parse("1-2-120, 2-3-130, 3-4-50").unwrap();

        assert_eq!(1, gtid_list1.gtids.len());
        assert_eq!(2, gtid_list2.gtids.len());
        assert_eq!(3, gtid_list3.gtids.len());

        assert_eq!(String::from("0-1-270"), gtid_list1.to_string());
        assert_eq!(String::from("1-2-120,2-3-130"), gtid_list2.to_string());
        assert_eq!(
            String::from("1-2-120,2-3-130,3-4-50"),
            gtid_list3.to_string()
        );
    }

    #[test]
    fn add_existing_domain_gtid_updated() {
        let mut gtid_list = GtidList::parse("0-1-270").unwrap();
        gtid_list.add_gtid(Gtid::new(0, 1, 271));

        assert_eq!(1, gtid_list.gtids.len());
        assert_eq!(String::from("0-1-271"), gtid_list.to_string());
    }

    #[test]
    fn add_new_domain_gtid_added() {
        let mut gtid_list = GtidList::parse("0-1-270").unwrap();
        gtid_list.add_gtid(Gtid::new(1, 1, 271));

        assert_eq!(2, gtid_list.gtids.len());
        assert_eq!(String::from("0-1-270,1-1-271"), gtid_list.to_string());
    }

    #[test]
    fn add_multi_domain_gtid_list_merged() {
        let mut gtid_list = GtidList::parse("1-2-120,2-3-130,3-4-50").unwrap();
        gtid_list.add_gtid(Gtid::new(2, 4, 250));

        assert_eq!(3, gtid_list.gtids.len());
        assert_eq!(
            String::from("1-2-120,2-4-250,3-4-50"),
            gtid_list.to_string()
        );
    }
}
