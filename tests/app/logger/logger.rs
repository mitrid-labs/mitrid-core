use mitrid_core::app::Logger as BasicLogger;
use mitrid_core::app::{LogLevel, LogFile};

use fixtures::app::Logger;

#[test]
fn test_logger_new() {
    let logger = Logger::new();

    let res = logger.log_level();
    assert!(res.is_err());

    let res = logger.log_file();
    assert!(res.is_err());
}

#[test]
fn test_logger_log_level() {
    let mut logger = Logger::new();

    let res = logger.log_level();
    assert!(res.is_err());

    let log_level = LogLevel::default();
    let res = logger.set_log_level(&log_level);
    assert!(res.is_ok());

    let res = logger.log_level();
    assert!(res.is_ok());

    let found_log_level = res.unwrap();
    assert_eq!(found_log_level, log_level);
}

#[test]
fn test_logger_log_file() {
    let mut logger = Logger::new();

    let res = logger.log_file();
    assert!(res.is_err());

    let log_file = LogFile::default();
    let res = logger.set_log_file(&log_file);
    assert!(res.is_ok());

    let res = logger.log_file();
    assert!(res.is_ok());

    let found_log_file = res.unwrap();
    assert_eq!(found_log_file, log_file);
}