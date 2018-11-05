//! # Util
//!
//! `util` is the module providing utility types and helper functions used across the library.

/// Types used to produce and manipulate timestamps.
pub mod timestamp;

/// Functions used to match and pattern match strings against a regex.
pub mod regex;

/// Type and constants used for versioning (semver).
pub mod version;

pub use self::timestamp::{Timestamp, TimestampDiff};
pub use self::version::Version;