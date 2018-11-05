use tempfile::tempdir;
use mitrid_core::app::Logger as BasicLogger;
use mitrid_core::app::{LogLevel, LogFile};

use std::fs::{self, File};

use fixture::app::Logger;

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

#[test]
fn test_logger_log_error() {
    let log_file_name = "log_error_test";
    let temp_dir = tempdir().unwrap();

    let log_path = format!("{}", temp_dir.path().join(log_file_name).to_str().unwrap());
    let file = File::create(&log_path).unwrap();
    
    let res = file.metadata();
    assert!(res.is_ok());

    let metadata = res.unwrap();
    assert!(metadata.is_file());

    let mut logger = Logger::new();
    let log_level = LogLevel::Error;
    let log_file = LogFile::parse(&log_path);

    logger.set_log_level(&log_level).unwrap();
    logger.set_log_file(&log_file).unwrap();

    let content = "this is an error";
    let content_a = format!("error: {}\n", content);

    let res = logger.log_error(&content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let content_b = String::from_utf8(buf).unwrap();

    assert_eq!(content_a, content_b);

    drop(file);
    temp_dir.close().unwrap();
}

#[test]
fn test_logger_log_warn() {
    let log_file_name = "log_warn_test";
    let temp_dir = tempdir().unwrap();

    let log_path = format!("{}", temp_dir.path().join(log_file_name).to_str().unwrap());
    let file = File::create(&log_path).unwrap();

    let mut logger = Logger::new();
    let log_level = LogLevel::Warn;
    let log_file = LogFile::parse(&log_path);

    logger.set_log_level(&log_level).unwrap();
    logger.set_log_file(&log_file).unwrap();

    let content = "this is a warning";
    let content_a = format!("warning: {}\n", content);

    let res = logger.log_warn(&content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let content_b = String::from_utf8(buf).unwrap();

    assert_eq!(content_a, content_b);

    drop(file);
    temp_dir.close().unwrap();
}

#[test]
fn test_logger_log_info() {
    let log_file_name = "log_info_test";
    let temp_dir = tempdir().unwrap();

    let log_path = format!("{}", temp_dir.path().join(log_file_name).to_str().unwrap());
    let file = File::create(&log_path).unwrap();

    let mut logger = Logger::new();
    let log_level = LogLevel::Info;
    let log_file = LogFile::parse(&log_path);

    logger.set_log_level(&log_level).unwrap();
    logger.set_log_file(&log_file).unwrap();

    let info_content = "this is an information";
    let info_content_a = format!("info: {}\n", info_content);

    let res = logger.log_info(&info_content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let info_content_b = String::from_utf8(buf).unwrap();

    assert_eq!(info_content_a, info_content_b);

    let warn_content = "this is a warning";
    let warn_content_a = format!("{}warning: {}\n", info_content_a, warn_content);

    let res = logger.log_warn(&warn_content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let warn_content_b = String::from_utf8(buf).unwrap();

    assert_eq!(warn_content_a, warn_content_b);

    drop(file);
    temp_dir.close().unwrap();
}

#[test]
fn test_logger_log_debug() {
    let log_file_name = "log_debug_test";
    let temp_dir = tempdir().unwrap();

    let log_path = format!("{}", temp_dir.path().join(log_file_name).to_str().unwrap());
    let file = File::create(&log_path).unwrap();

    let mut logger = Logger::new();
    let log_level = LogLevel::Debug;
    let log_file = LogFile::parse(&log_path);

    logger.set_log_level(&log_level).unwrap();
    logger.set_log_file(&log_file).unwrap();

    let debug_content = "this is a debug log";
    let debug_content_a = format!("debug: {}\n", debug_content);

    let res = logger.log_debug(&debug_content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let debug_content_b = String::from_utf8(buf).unwrap();

    assert_eq!(debug_content_a, debug_content_b);

    let info_content = "this is an information";
    let info_content_a = format!("{}info: {}\n", debug_content_a, info_content);

    let res = logger.log_info(&info_content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let info_content_b = String::from_utf8(buf).unwrap();

    assert_eq!(info_content_a, info_content_b);

    let warn_content = "this is a warning";
    let warn_content_a = format!("{}warning: {}\n", info_content_a, warn_content);

    let res = logger.log_warn(&warn_content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let warn_content_b = String::from_utf8(buf).unwrap();

    assert_eq!(warn_content_a, warn_content_b);

    drop(file);
    temp_dir.close().unwrap();
}

#[test]
fn test_logger_log_trace() {
    let log_file_name = "log_trace_test";
    let temp_dir = tempdir().unwrap();

    let log_path = format!("{}", temp_dir.path().join(log_file_name).to_str().unwrap());
    let file = File::create(&log_path).unwrap();

    let mut logger = Logger::new();
    let log_level = LogLevel::Trace;
    let log_file = LogFile::parse(&log_path);

    logger.set_log_level(&log_level).unwrap();
    logger.set_log_file(&log_file).unwrap();

    let trace_content = "this is a trace log";
    let trace_content_a = format!("trace: {}\n", trace_content);

    let res = logger.log_trace(&trace_content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let trace_content_b = String::from_utf8(buf).unwrap();

    assert_eq!(trace_content_a, trace_content_b);

    let debug_content = "this is a debug log";
    let debug_content_a = format!("{}debug: {}\n", trace_content_a, debug_content);

    let res = logger.log_debug(&debug_content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let debug_content_b = String::from_utf8(buf).unwrap();

    assert_eq!(debug_content_a, debug_content_b);

    let info_content = "this is an information";
    let info_content_a = format!("{}info: {}\n", debug_content_a, info_content);

    let res = logger.log_info(&info_content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let info_content_b = String::from_utf8(buf).unwrap();

    assert_eq!(info_content_a, info_content_b);

    let warn_content = "this is a warning";
    let warn_content_a = format!("{}warning: {}\n", info_content_a, warn_content);

    let res = logger.log_warn(&warn_content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let warn_content_b = String::from_utf8(buf).unwrap();

    assert_eq!(warn_content_a, warn_content_b);

    drop(file);
    temp_dir.close().unwrap();
}

#[test]
fn test_logger_log_all() {
    let log_file_name = "log";
    let temp_dir = tempdir().unwrap();

    let log_path = format!("{}", temp_dir.path().join(log_file_name).to_str().unwrap());
    let file = File::create(&log_path).unwrap();

    let mut logger = Logger::new();
    let log_level = LogLevel::All;
    let log_file = LogFile::parse(&log_path);

    logger.set_log_level(&log_level).unwrap();
    logger.set_log_file(&log_file).unwrap();

    let trace_content = "this is a trace log";
    let trace_content_a = format!("trace: {}\n", trace_content);

    let res = logger.log_trace(&trace_content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let trace_content_b = String::from_utf8(buf).unwrap();

    assert_eq!(trace_content_a, trace_content_b);

    let debug_content = "this is a debug log";
    let debug_content_a = format!("{}debug: {}\n", trace_content_a, debug_content);

    let res = logger.log_debug(&debug_content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let debug_content_b = String::from_utf8(buf).unwrap();

    assert_eq!(debug_content_a, debug_content_b);

    let info_content = "this is an information";
    let info_content_a = format!("{}info: {}\n", debug_content_a, info_content);

    let res = logger.log_info(&info_content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let info_content_b = String::from_utf8(buf).unwrap();

    assert_eq!(info_content_a, info_content_b);

    let warn_content = "this is a warning";
    let warn_content_a = format!("{}warning: {}\n", info_content_a, warn_content);

    let res = logger.log_warn(&warn_content);
    assert!(res.is_ok());

    let buf = fs::read(&log_path).unwrap();
    let warn_content_b = String::from_utf8(buf).unwrap();

    assert_eq!(warn_content_a, warn_content_b);

    drop(file);
    temp_dir.close().unwrap();
}