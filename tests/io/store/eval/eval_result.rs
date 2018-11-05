use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;

use fixture::io::Session;
use fixture::io::store::{StoreEvalResult, DumpSessions, DumpItems, DumpAll};

#[test]
fn test_eval_result_new_size() {
    let size = 1024;
    let result = StoreEvalResult::new_size(size);

    match result {
        StoreEvalResult::Size(result) => assert_eq!(result, size),
        _ => panic!("invalid variant"),
    }
}

#[test]
fn test_eval_result_new_dump_sessions() {
    let dump_sessions = DumpSessions::default();
    let res = StoreEvalResult::new_dump_sessions(&dump_sessions);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        StoreEvalResult::DumpSessions(result) => assert_eq!(result, dump_sessions),
        _ => panic!("invalid variant"),
    }
}

#[test]
fn test_eval_result_new_dump_items() {
    let dump_items = DumpItems::default();
    let res = StoreEvalResult::new_dump_items(&dump_items);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        StoreEvalResult::DumpItems(result) => assert_eq!(result, dump_items),
        _ => panic!("invalid variant"),
    }
}

#[test]
fn test_eval_result_new_dump_all() {
    let dump_all = DumpAll::default();
    let res = StoreEvalResult::new_dump_all(&dump_all);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        StoreEvalResult::DumpAll(result) => assert_eq!(result, dump_all),
        _ => panic!("invalid variant"),
    }
}

#[test]
fn test_eval_result_default() {
    let default_result = StoreEvalResult::default();
    assert_eq!(default_result, StoreEvalResult::DumpAll(DumpAll::default()));
}

#[test]
fn test_eval_result_size() {
    let result = StoreEvalResult::default();
    let result_size = result.size();
    let expected_size = DumpAll::default().size();
    assert_eq!(result_size, expected_size);
}

#[test]
fn test_eval_result_check() {
    let mut result = StoreEvalResult::default();
    let res = result.check();
    assert!(res.is_ok());

    let mut dump_sessions = DumpSessions::default();
    let session = Session::default();
    dump_sessions.sessions.push((session.id, session.clone()));

    result = StoreEvalResult::DumpSessions(dump_sessions);
    let res = result.check();
    assert!(res.is_err());

    let mut dump_items = DumpItems::default();
    dump_items.items.push((Vec::default(), Vec::default()));

    result = StoreEvalResult::DumpItems(dump_items);
    let res = result.check();
    assert!(res.is_err());

    let mut dump_all = DumpAll::default();
    dump_all.sessions_count += 1;
    dump_all.items_count += 1;

    result = StoreEvalResult::DumpAll(dump_all);
    let res = result.check();
    assert!(res.is_err());
}

#[test]
fn test_eval_result_serialize_json() {
    let result_a = StoreEvalResult::default();
    
    let res = result_a.to_json();
    assert!(res.is_ok());
    let json = res.unwrap();

    let res = StoreEvalResult::from_json(&json);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_eval_result_serialize_bytes() {
    let result_a = StoreEvalResult::default();
    
    let res = result_a.to_bytes();
    assert!(res.is_ok());
    let bytes = res.unwrap();

    let res = StoreEvalResult::from_bytes(&bytes);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_eval_result_serialize_hex() {
    let result_a = StoreEvalResult::default();
    
    let res = result_a.to_hex();
    assert!(res.is_ok());
    let hex = res.unwrap();

    let res = StoreEvalResult::from_hex(&hex);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}