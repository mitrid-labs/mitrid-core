//! # Env
//!
//! `env` is the module providing the trait implemented by types used to read the process environment.

use std::env;
use std::path::PathBuf;
use std::collections::HashMap;

use base::Result;
use app::logger::{LogLevel, LogFile};

/// Trait implemented by types used to read the process environment.
pub trait Env
    where   Self: 'static + Sized + Send + Sync
{
    /// Returns the current variables.
    fn vars(&self) -> Result<HashMap<String, String>> {
        let mut vars: HashMap<String, String> = HashMap::new();

        for (key, var) in env::vars() {
            vars.insert(key.into(), var.into());
        }

        Ok(vars)
    }

    /// Returns the current arguments.
    fn args(&self) -> Result<Vec<String>> {
        let mut args: Vec<String> = Vec::new();

        for arg in env::args() {
            args.push(arg)
        }

        Ok(args)
    }

    /// Returns the current directory.
    fn current_dir(&self) -> Result<PathBuf> {
        env::current_dir().map_err(|e| format!("{}", e))
    }

    /// Returns the current log level.
    fn log_level(&self) -> Result<LogLevel> {
        let vars = self.vars()?;

        if vars.contains_key("LOG_LEVEL") {
            let value = vars.get("LOG_LEVEL").unwrap();
            return LogLevel::parse(&value);
        }

        if vars.contains_key("log_level") {
            let value = vars.get("log_level").unwrap();
            return LogLevel::parse(&value);
        }
        
        Ok(LogLevel::None)
    }

    /// Returns the current log file.
    fn log_file(&self) -> Result<LogFile> {
        let vars = self.vars()?;

        if vars.contains_key("LOG_FILE") {
            let value = vars.get("LOG_FILE").unwrap();
            return Ok(LogFile::parse(&value));
        }

        if vars.contains_key("log_file") {
            let value = vars.get("log_file").unwrap();
            return Ok(LogFile::parse(&value));
        }
        
        Ok(LogFile::StdOut)
    }
}