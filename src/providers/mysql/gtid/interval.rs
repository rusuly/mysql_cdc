use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents contiguous transaction interval in GtidSet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interval {
    /// Gets first transaction id in the interval.
    pub start: u64,

    /// Gets last transaction id in the interval.
    pub end: u64,
}

impl Interval {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}

impl fmt::Display for Interval {
    /// Returns string representation of an UuidSet interval.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}
