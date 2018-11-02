use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::app::logger::LogFile;

#[test]
fn test_log_file_parse() {
    let valid_log_file_strs = vec!["stdout", "stderr", "path1", "path2", "path3", "path4"];

    for log_file_str in valid_log_file_strs.iter() {
        LogFile::parse(log_file_str);
    }
}

#[test]
fn test_log_file_display() {
    let log_file_strs = vec!["stdout", "stderr", "path1", "path2", "path3", "path4"];

    for log_file_str in log_file_strs.iter() {
        let log_file = LogFile::parse(log_file_str);
        assert_eq!(format!("{}", log_file), String::from(*log_file_str));
    }
}

#[test]
fn test_log_file_default() {
    let log_file = LogFile::default();
    assert_eq!(log_file, LogFile::StdOut);
}

#[test]
fn test_log_file_size() {
    let log_file = LogFile::default();
    assert_eq!(log_file.size(), 0u8.size());

    let path = String::from("this is a path");
    let log_file = LogFile::Path(path.clone());
    assert_eq!(log_file.size(), path.size());
}

#[test]
fn test_log_file_serialize_json() {
    let log_file_a = LogFile::default();
    let res = log_file_a.to_json();
    assert!(res.is_ok());

    let log_file_a_json = res.unwrap();
    let res = LogFile::from_json(&log_file_a_json);
    assert!(res.is_ok());

    let log_file_b = res.unwrap();
    assert_eq!(log_file_a, log_file_b);
}

#[test]
fn test_log_file_serialize_bytes() {
    let log_file_a = LogFile::default();
    let res = log_file_a.to_bytes();
    assert!(res.is_ok());

    let log_file_a_bytes = res.unwrap();
    let res = LogFile::from_bytes(&log_file_a_bytes);
    assert!(res.is_ok());

    let log_file_b = res.unwrap();
    assert_eq!(log_file_a, log_file_b);
}

#[test]
fn test_log_file_serialize_hex() {
    let log_file_a = LogFile::default();
    let res = log_file_a.to_hex();
    assert!(res.is_ok());

    let log_file_a_hex = res.unwrap();
    let res = LogFile::from_hex(&log_file_a_hex);
    assert!(res.is_ok());

    let log_file_b = res.unwrap();
    assert_eq!(log_file_a, log_file_b);
}