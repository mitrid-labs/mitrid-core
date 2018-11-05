//! # Timestamp
//!
//! `timestamp` is the module providing the timestamping utilities.

pub mod timestamp_diff;
pub mod timestamp;

pub use self::timestamp_diff::TimestampDiff;
pub use self::timestamp::Timestamp;