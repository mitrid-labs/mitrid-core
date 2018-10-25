//! # LogLevel
//!
//! `log_level` is the module providing the type used to represent the log level.

use std::fmt;

use base::Result;
use base::Checkable;
use base::Sizable;
use base::Datable;
use base::Serializable;

/// Type used to represent the current log level.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum LogLevel {
    /// No logging.
    None,
    /// Logs errors.
    Error,
    /// Logs warnings.
    Warn,
    /// Logs information.
    Info,
    /// Logs debug information.
    Debug,
    /// Logs tracing information.
    Trace,
    /// Logs all.
    All
}

impl LogLevel {
    /// Parses a `LogLevel` from a `&str`.
    pub fn parse(s: &str) -> Result<LogLevel> {
        match s.to_lowercase().as_str() {
            "none" => Ok(LogLevel::None),
            "error" => Ok(LogLevel::Error),
            "warn" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            "all" => Ok(LogLevel::All),
            _ => Err("unknown log level".into())
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::None => write!(f, "none"),
            LogLevel::Error => write!(f, "error"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::All => write!(f, "all"),
        }
    }
}

impl Default for LogLevel {
    fn default() -> LogLevel {
        LogLevel::None
    }
}

impl Sizable for LogLevel {
    fn size(&self) -> u64 {
        0u8.size()
    }
}

impl Checkable for LogLevel {}

impl Serializable for LogLevel {}

impl Datable for LogLevel {}