use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;

use fixture::io::Session;
use fixture::io::store::eval::DumpSessions;

#[test]
fn test_dump_sessions_new() {
    let sessions = vec![Session::default()];
    let res = DumpSessions::new(&sessions);
    assert!(res.is_ok());

    let result = res.unwrap();
    assert_eq!(result.count, result.sessions.len() as u64);
}

#[test]
fn test_dump_sessions_default() {
    let default_result = DumpSessions::default();
    let expected_result = DumpSessions {
        count: 0,
        sessions: Vec::new(),
    };
    assert_eq!(default_result, expected_result);
}

#[test]
fn test_dump_sessions_size() {
    let result = DumpSessions::default();
    let result_size = result.size();
    let expected_size = result.count.size() + result.sessions.size();
    assert_eq!(result_size, expected_size);
}

#[test]
fn test_dump_sessions_check() {
    let mut result = DumpSessions::default();
    let res = result.check();
    assert!(res.is_ok());

    result.count += 1;
    let res = result.check();
    assert!(res.is_err());

    let session = Session::default();
    result.sessions.push((session.id, session.clone()));
    let res = result.check();
    assert!(res.is_ok());

    result.sessions[0].0 = session.id + 1;
    let res = result.check();
    assert!(res.is_err());
}

#[test]
fn test_dump_sessions_serialize_json() {
    let result_a = DumpSessions::default();
    
    let res = result_a.to_json();
    assert!(res.is_ok());
    let json = res.unwrap();

    let res = DumpSessions::from_json(&json);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_dump_sessions_serialize_bytes() {
    let result_a = DumpSessions::default();
    
    let res = result_a.to_bytes();
    assert!(res.is_ok());
    let bytes = res.unwrap();

    let res = DumpSessions::from_bytes(&bytes);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_dump_sessions_serialize_hex() {
    let result_a = DumpSessions::default();
    
    let res = result_a.to_hex();
    assert!(res.is_ok());
    let hex = res.unwrap();

    let res = DumpSessions::from_hex(&hex);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}