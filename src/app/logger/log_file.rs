//! # LogFile
//!
//! `log_file` is the module providing the type used to represent the log file.

use std::fmt;
use std::fs::metadata;

use base::Result;
use base::Checkable;
use base::Sizable;
use base::Datable;
use base::Serializable;

/// Type used to represent the current log file.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[allow(unused_attributes)]
pub enum LogFile {
    /// Logs to stdout.
    #[repr(u8)]
    StdOut,
    /// Logs to stderr.
    #[repr(u8)]
    StdErr,
    /// Logs to a file system file.
    Path(String),
}

impl LogFile {
    /// Parses a `LogFile` from a `&str`.
    pub fn parse(s: &str) -> LogFile {
        match s.to_lowercase().as_str() {
            "stdout" => LogFile::StdOut,
            "stderr" => LogFile::StdErr,
            path => LogFile::Path(path.into()),
        }
    }
}

impl fmt::Display for LogFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogFile::StdOut => write!(f, "stdout"),
            LogFile::StdErr => write!(f, "stderr"),
            LogFile::Path(ref path) => write!(f, "{}", path),
        }
    }
}

impl Default for LogFile {
    fn default() -> LogFile {
        LogFile::StdOut
    }
}

impl Sizable for LogFile {
    fn size(&self) -> u64 {
        match self {
            LogFile::StdOut => 0u8.size(),
            LogFile::StdErr => 0u8.size(),
            LogFile::Path(ref path) => path.size(),
        }
    }
}

impl Checkable for LogFile {
    fn check(&self) -> Result<()> {
        match self {
            &LogFile::StdOut => Ok(()),
            &LogFile::StdErr => Ok(()),
            &LogFile::Path(ref path) => {
                metadata(path)
                    .map(|_| ())
                    .map_err(|e| format!("{}", e))
            },
        }
    }
}

impl Serializable for LogFile {}

impl Datable for LogFile {}