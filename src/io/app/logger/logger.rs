//! # Logger
//!
//! `logger` is the module providing the trait implemented by types that can log to stdout/stderr.

use base::Result;
use io::app::logger::LogLevel;

/// Trait implemented by types that can log to stdout/stderr.
pub trait Logger {
    fn log_level(&self) -> LogLevel;

    fn log_error(&self, content: &str) -> Result<()>;

    fn log_warn(&self, content: &str) -> Result<()>;
    
    fn log_info(&self, content: &str) -> Result<()>;
    
    fn log_debug(&self, content: &str) -> Result<()>;
    
    fn log_trace(&self, content: &str) -> Result<()>;
}