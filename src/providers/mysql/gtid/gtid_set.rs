use crate::providers::mysql::gtid::gtid::Gtid;
use crate::providers::mysql::gtid::interval::Interval;
use crate::providers::mysql::gtid::uuid::Uuid;
use crate::providers::mysql::gtid::uuid_set::UuidSet;
use std::collections::HashMap;
use std::fmt;

const UUID_LENGTH: usize = 36;

/// Represents GtidSet from MySQL 5.6 and above.
/// <a href="https://dev.mysql.com/doc/refman/8.0/en/replication-gtids-concepts.html">See more</a>
#[derive(Debug)]
pub struct GtidSet {
    /// Gets a list of UuidSet parts in the GtidSet.
    pub uuid_sets: HashMap<String, UuidSet>,
}

impl GtidSet {
    pub fn new() -> Self {
        Self {
            uuid_sets: HashMap::new(),
        }
    }

    /// Parses <see cref="GtidSet"/> from string representation.
    pub fn parse(gtid_set: &str) -> Self {
        if gtid_set.is_empty() {
            return GtidSet::new();
        }

        let gtid_set = gtid_set.replace("\n", "");
        let uuid_sets = gtid_set.split(',').map(|x| x.trim()).collect::<Vec<&str>>();

        let mut result = HashMap::new();
        for uuid_set in uuid_sets {
            let source_id: String = uuid_set.chars().take(UUID_LENGTH).collect();
            let source_id = Uuid::parse(source_id);

            let mut intervals = Vec::new();
            let ranges: String = uuid_set.chars().skip(UUID_LENGTH + 1).collect();
            let ranges = ranges.split(':').collect::<Vec<&str>>();

            for token in ranges {
                let range = token.split('-').collect::<Vec<&str>>();
                let interval = match range.len() {
                    1 => Interval::new(range[0].parse().unwrap(), range[0].parse().unwrap()),
                    2 => Interval::new(range[0].parse().unwrap(), range[1].parse().unwrap()),
                    _ => panic!("Invalid interval format {}", token),
                };
                intervals.push(interval);
            }
            result.insert(source_id.uuid.clone(), UuidSet::new(source_id, intervals));
        }

        Self { uuid_sets: result }
    }

    /// Adds a gtid value to the GtidSet.
    pub fn add_gtid(&mut self, gtid: Gtid) -> bool {
        let uuid_set = self
            .uuid_sets
            .entry(gtid.source_id.uuid.clone())
            .or_insert(UuidSet::new(gtid.source_id.clone(), Vec::new()));

        uuid_set.add_gtid(gtid)
    }
}

impl fmt::Display for GtidSet {
    /// Returns string representation of the GtidSet.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = self
            .uuid_sets
            .values()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        write!(f, "{}", str)
    }
}
