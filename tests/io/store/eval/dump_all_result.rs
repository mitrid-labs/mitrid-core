use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;

use fixture::io::Session;
use fixture::io::store::eval::DumpAll;

#[test]
fn test_dump_all_new() {
    let sessions = vec![Session::default()];
    let items = vec![(Vec::default(), Vec::default())];
    
    let res = DumpAll::new(&sessions, &items);
    assert!(res.is_ok());

    let result = res.unwrap();

    assert_eq!(result.sessions_count, result.sessions.len() as u64);
    assert_eq!(result.items_count, result.items.len() as u64);
}

#[test]
fn test_dump_all_default() {
    let default_result = DumpAll::default();
    let expected_result = DumpAll {
        sessions_count: 0,
        sessions: Vec::new(),
        items_count: 0,
        items: Vec::new(),
    };
    assert_eq!(default_result, expected_result);
}

#[test]
fn test_dump_all_size() {
    let result = DumpAll::default();
    let result_size = result.size();
    let expected_size = result.sessions_count.size() +
                        result.sessions.size() +
                        result.items_count.size() +
                        result.items.size();
    assert_eq!(result_size, expected_size);
}

#[test]
fn test_dump_all_check() {
    let mut result = DumpAll::default();
    let res = result.check();
    assert!(res.is_ok());

    result.sessions_count += 1;
    let res = result.check();
    assert!(res.is_err());

    let session = Session::default();
    result.sessions.push((session.id, session.clone()));
    let res = result.check();
    assert!(res.is_ok());

    result.items_count += 1;
    let res = result.check();
    assert!(res.is_err());

    result.items.push((Vec::default(), Vec::default()));
    let res = result.check();
    assert!(res.is_ok());
}

#[test]
fn test_dump_all_serialize_json() {
    let result_a = DumpAll::default();
    
    let res = result_a.to_json();
    assert!(res.is_ok());
    let json = res.unwrap();

    let res = DumpAll::from_json(&json);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_dump_all_serialize_bytes() {
    let result_a = DumpAll::default();
    
    let res = result_a.to_bytes();
    assert!(res.is_ok());
    let bytes = res.unwrap();

    let res = DumpAll::from_bytes(&bytes);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_dump_all_serialize_hex() {
    let result_a = DumpAll::default();
    
    let res = result_a.to_hex();
    assert!(res.is_ok());
    let hex = res.unwrap();

    let res = DumpAll::from_hex(&hex);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}