use crate::errors::Error;
use crate::providers::mysql::gtid::gtid::Gtid;
use crate::providers::mysql::gtid::interval::Interval;
use crate::providers::mysql::gtid::uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents replication state for a specific server.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UuidSet {
    /// Gets server uuid of the UuidSet.
    pub source_id: Uuid,

    /// Gets a list of intervals of the UuidSet.
    pub intervals: Vec<Interval>,
}

impl UuidSet {
    pub fn new(source_id: Uuid, mut intervals: Vec<Interval>) -> Self {
        if intervals.len() > 1 {
            collapse_intervals(&mut intervals);
        }
        Self {
            source_id,
            intervals,
        }
    }

    /// Adds a gtid value to the UuidSet.
    pub fn add_gtid(&mut self, gtid: Gtid) -> Result<bool, Error> {
        if self.source_id.data != gtid.source_id.data {
            return Err(Error::String(
                "SourceId of the passed gtid doesn't belong to the UuidSet".to_string(),
            ));
        }

        let index = find_interval_index(&self.intervals, gtid.transaction_id);
        let mut added = false;
        if index < self.intervals.len() {
            let interval = &mut self.intervals[index];
            if interval.start == gtid.transaction_id + 1 {
                interval.start = gtid.transaction_id;
                added = true;
            } else if interval.end + 1 == gtid.transaction_id {
                interval.end = gtid.transaction_id;
                added = true;
            } else if interval.start <= gtid.transaction_id && gtid.transaction_id <= interval.end {
                return Ok(false);
            }
        }
        if !added {
            let interval = Interval::new(gtid.transaction_id, gtid.transaction_id);
            self.intervals.insert(index, interval);
        }
        if self.intervals.len() > 1 {
            collapse_intervals(&mut self.intervals);
        }
        Ok(true)
    }
}

pub fn find_interval_index(intervals: &Vec<Interval>, transaction_id: u64) -> usize {
    let mut result_index = 0;
    let mut left_index = 0;
    let mut right_index = intervals.len();

    while left_index < right_index {
        result_index = (left_index + right_index) / 2;
        let interval = &intervals[result_index];
        if interval.end < transaction_id {
            left_index = result_index + 1;
        } else if transaction_id < interval.start {
            right_index = result_index;
        } else {
            return result_index;
        }
    }
    if !intervals.is_empty() && intervals[result_index].end < transaction_id {
        result_index += 1;
    }
    result_index
}

pub fn collapse_intervals(intervals: &mut Vec<Interval>) {
    let mut index = 0;

    while index < intervals.len() - 1 {
        let right_start = intervals[index + 1].start;
        let right_end = intervals[index + 1].end;

        let mut left = &mut intervals[index];
        if left.end + 1 == right_start {
            left.end = right_end;
            intervals.remove(index + 1);
        } else {
            index += 1;
        }
    }
}

impl fmt::Display for UuidSet {
    /// Returns string representation of an UuidSet part of a GtidSet.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let intervals = self
            .intervals
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(":");

        write!(f, "{}:{}", self.source_id, intervals)
    }
}
