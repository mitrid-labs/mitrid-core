//! # Env
//!
//! `env` is the module providing the trait implemented by types used to read the process environment.

use std::env;
use std::path::PathBuf;
use std::collections::HashMap;

use base::Result;
use app::logger::LogLevel;

/// Trait implemented by types used to read the process environment.
pub trait Env
    where   Self: 'static + Sized + Send + Sync
{
    /// Returns the current arguments.
    fn args(&self) -> Result<HashMap<String, String>>;

    /// Returns the current directory.
    fn current_dir(&self) -> Result<PathBuf> {
        env::current_dir().map_err(|e| format!("{}", e))
    }

    /// Returns the environment log level.
    fn log_level(&self) -> Result<LogLevel> {
        let args = self.args()?;

        if args.contains_key("LOG_LEVEL") {
            let value = args.get("LOG_LEVEL").unwrap();
            return LogLevel::parse(&value);
        }

        if args.contains_key("log_level") {
            let value = args.get("log_level").unwrap();
            return LogLevel::parse(&value);
        }
        
        Ok(LogLevel::None)
    }
}