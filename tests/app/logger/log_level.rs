use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::app::logger::LogLevel;

#[test]
fn test_log_level_parse() {
    let valid_log_level_strs = vec!["none", "error", "warn", "info", "debug", "trace", "all"];
    let invalid_log_level_str = "fail";

    for log_level_str in valid_log_level_strs.iter() {
        let res = LogLevel::parse(log_level_str);
        assert!(res.is_ok());
    }

    let res = LogLevel::parse(invalid_log_level_str);
    assert!(res.is_err());
}

#[test]
fn test_log_level_display() {
    let log_level_strs = vec!["none", "error", "warn", "info", "debug", "trace", "all"];

    for log_level_str in log_level_strs.iter() {
        let log_level = LogLevel::parse(log_level_str).unwrap();
        assert_eq!(format!("{}", log_level), String::from(*log_level_str));
    }
}

#[test]
fn test_log_level_default() {
    let log_level = LogLevel::default();
    assert_eq!(log_level, LogLevel::None);
}

#[test]
fn test_log_level_size() {
    let log_level = LogLevel::default();
    let log_level_size = (log_level as u8).size();
    assert_eq!(log_level.size(), log_level_size)
}

#[test]
fn test_log_level_serialize_json() {
    let log_level_a = LogLevel::default();
    let res = log_level_a.to_json();
    assert!(res.is_ok());

    let log_level_a_json = res.unwrap();
    let res = LogLevel::from_json(&log_level_a_json);
    assert!(res.is_ok());

    let log_level_b = res.unwrap();
    assert_eq!(log_level_a, log_level_b);
}

#[test]
fn test_log_level_serialize_bytes() {
    let log_level_a = LogLevel::default();
    let res = log_level_a.to_bytes();
    assert!(res.is_ok());

    let log_level_a_bytes = res.unwrap();
    let res = LogLevel::from_bytes(&log_level_a_bytes);
    assert!(res.is_ok());

    let log_level_b = res.unwrap();
    assert_eq!(log_level_a, log_level_b);
}

#[test]
fn test_log_level_serialize_hex() {
    let log_level_a = LogLevel::default();
    let res = log_level_a.to_hex();
    assert!(res.is_ok());

    let log_level_a_hex = res.unwrap();
    let res = LogLevel::from_hex(&log_level_a_hex);
    assert!(res.is_ok());

    let log_level_b = res.unwrap();
    assert_eq!(log_level_a, log_level_b);
}