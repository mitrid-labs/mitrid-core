use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::app::{LogLevel, LogFile};
use mitrid_core::app::Logger as BasicLogger;

use std::io::{self, Write};
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Clone, Default)]
pub struct Logger {
    log_level: Option<LogLevel>,
    log_file: Option<LogFile>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger::default()
    }

    fn write_stdout(&self, content: &str) -> Result<()> {
        io::stdout().write_all(content.as_bytes())
            .map_err(|e| format!("{:?}", e))
    }

    fn write_stderr(&self, content: &str) -> Result<()> {
        io::stderr().write_all(content.as_bytes())
            .map_err(|e| format!("{:?}", e))
    }

    fn write_file<P: AsRef<Path>>(&self, path: &P, content: &str) -> Result<()> {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .or_else(|e| {
                Err(format!("{}", e))
            })
            .and_then(|mut file| {
                let content = format!("{}\n", content);
                file.write_all(content.as_bytes())
                    .map_err(|e| format!("{:?}", e))
            })
    }
}

impl BasicLogger for Logger {
    fn log_level(&self) -> Result<LogLevel> {
        match self.log_level {
            Some(log_level) => Ok(log_level),
            None => Err(format!("not found"))
        }
    }

    fn set_log_level(&mut self, log_level: &LogLevel) -> Result<()> {
        log_level.check()?;

        self.log_level = Some(log_level.to_owned());

        Ok(())
    }

    fn log_file(&self) -> Result<LogFile> {
        match self.log_file.clone() {
            Some(log_file) => Ok(log_file),
            None => Err(format!("not found"))
        }
    }

    fn set_log_file(&mut self, log_file: &LogFile) -> Result<()> {
        log_file.check()?;

        self.log_file = Some(log_file.to_owned());

        Ok(())
    }

    fn log_error(&self, content: &str) -> Result<()> {
        let msg = format!("error: {}", content);
        let log_level = self.log_level()?;

        if log_level < LogLevel::Error {
            return Ok(())
        }

        match self.log_file()? {
            LogFile::StdOut => {
                self.write_stdout(&msg)
            },
            LogFile::StdErr => {
                self.write_stderr(&msg)
            },
            LogFile::Path(ref path) => {
                self.write_file(path, &msg)
            },
        }
    }

    fn log_warn(&self, content: &str) -> Result<()> {
        let msg = format!("warning: {}", content);
        let log_level = self.log_level()?;

        if log_level < LogLevel::Warn {
            return Ok(())
        }

        match self.log_file()? {
            LogFile::StdOut => {
                self.write_stdout(&msg)
            },
            LogFile::StdErr => {
                self.write_stderr(&msg)
            },
            LogFile::Path(ref path) => {
                self.write_file(path, &msg)
            },
        }
    }
    
    fn log_info(&self, content: &str) -> Result<()> {
        let msg = format!("info: {}", content);
        let log_level = self.log_level()?;

        if log_level < LogLevel::Info {
            return Ok(())
        }

        match self.log_file()? {
            LogFile::StdOut => {
                self.write_stdout(&msg)
            },
            LogFile::StdErr => {
                self.write_stderr(&msg)
            },
            LogFile::Path(ref path) => {
                self.write_file(path, &msg)
            },
        }
    }
    
    fn log_debug(&self, content: &str) -> Result<()> {
        let msg = format!("debug: {}", content);
        let log_level = self.log_level()?;

        if log_level < LogLevel::Debug {
            return Ok(())
        }

        match self.log_file()? {
            LogFile::StdOut => {
                self.write_stdout(&msg)
            },
            LogFile::StdErr => {
                self.write_stderr(&msg)
            },
            LogFile::Path(ref path) => {
                self.write_file(path, &msg)
            },
        }
    }
    
    fn log_trace(&self, content: &str) -> Result<()> {
        let msg = format!("trace: {}", content);
        let log_level = self.log_level()?;

        if log_level < LogLevel::Trace {
            return Ok(())
        }

        match self.log_file()? {
            LogFile::StdOut => {
                self.write_stdout(&msg)
            },
            LogFile::StdErr => {
                self.write_stderr(&msg)
            },
            LogFile::Path(ref path) => {
                self.write_file(path, &msg)
            },
        }
    }
}