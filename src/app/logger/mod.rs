//! # Logger
//!
//! `logger` is the module providing the traits and types used for logging to stdout/stderr.

/// Type used to represent the log level.
pub mod log_level;

/// Type used to represent the log file.
pub mod log_file;

/// Trait implemented by types used to log to stdout/stderr.
pub mod logger;

pub use self::log_level::LogLevel;
pub use self::log_file::LogFile;
pub use self::logger::Logger;