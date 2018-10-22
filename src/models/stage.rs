//! # Stage
//!
//! `stage` is the module providing the type describing the distributed ledger stage (development,
//! testing or production).

use std::fmt;

use base::Result;
use base::Checkable;
use base::Sizable;
use base::Datable;
use base::Serializable;

/// Enum representing the distributed ledger stage (development, testing or production).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Stage {
    Development,
    Testing,
    Production,
}

impl Stage {
    /// Parses a `Stage` from a `&str`.
    pub fn parse(s: &str) -> Result<Stage> {
        match s {
            "development" => Ok(Stage::Development),
            "testing" => Ok(Stage::Testing),
            "production" => Ok(Stage::Production),
            _ => Err("unknown stage".into())
        }
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stage::Development => write!(f, "development"),
            Stage::Testing => write!(f, "testing"),
            Stage::Production => write!(f, "production"),
        }
    }
}

impl Default for Stage {
    fn default() -> Stage {
        Stage::Development
    }
}

impl Sizable for Stage {
    fn size(&self) -> u64 {
        0u8.size()
    }
}

impl Checkable for Stage {}

impl Serializable for Stage {}

impl Datable for Stage {}