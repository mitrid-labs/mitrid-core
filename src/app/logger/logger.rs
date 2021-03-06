//! # Logger
//!
//! `logger` is the module providing the trait implemented by types that can log to stdout/stderr.

use base::Result;
use app::LogLevel;
use app::LogFile;

/// Trait implemented by types that can log to stdout/stderr.
pub trait Logger
    where   Self: 'static + Sized + Send + Sync
{
    /// Returns the current log level.
    fn log_level(&self) -> Result<LogLevel>;

    /// Sets the current log level.
    fn set_log_level(&mut self, log_level: &LogLevel) -> Result<()>;

    /// Returns the current log file.
    fn log_file(&self) -> Result<LogFile>;

    /// Sets the current log file.
    fn set_log_file(&mut self, log_file: &LogFile) -> Result<()>;

    /// Logs an error.
    fn log_error(&self, content: &str) -> Result<()>;

    /// Logs a warning.
    fn log_warn(&self, content: &str) -> Result<()>;
    
    /// Logs an information.
    fn log_info(&self, content: &str) -> Result<()>;
    
    /// Logs a debug information.
    fn log_debug(&self, content: &str) -> Result<()>;
    
    /// Logs a trace information.
    fn log_trace(&self, content: &str) -> Result<()>;
}