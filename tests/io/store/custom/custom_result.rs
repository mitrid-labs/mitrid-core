use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;

use fixtures::io::Session;
use fixtures::io::store::{StoreKey, StoreValue};
use fixtures::io::store::custom::{CustomResult, DumpSessions, DumpItems, DumpAll};

#[test]
fn test_custom_result_new_size() {
    let size = 1024;
    let result = CustomResult::new_size(size);

    match result {
        CustomResult::Size(result) => assert_eq!(result, size),
        _ => panic!("invalid variant"),
    }
}

#[test]
fn test_custom_result_new_dump_sessions() {
    let dump_sessions = DumpSessions::default();
    let res = CustomResult::new_dump_sessions(&dump_sessions);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        CustomResult::DumpSessions(result) => assert_eq!(result, dump_sessions),
        _ => panic!("invalid variant"),
    }
}

#[test]
fn test_custom_result_new_dump_items() {
    let dump_items = DumpItems::default();
    let res = CustomResult::new_dump_items(&dump_items);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        CustomResult::DumpItems(result) => assert_eq!(result, dump_items),
        _ => panic!("invalid variant"),
    }
}

#[test]
fn test_custom_result_new_dump_all() {
    let dump_all = DumpAll::default();
    let res = CustomResult::new_dump_all(&dump_all);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        CustomResult::DumpAll(result) => assert_eq!(result, dump_all),
        _ => panic!("invalid variant"),
    }
}

#[test]
fn test_custom_result_default() {
    let default_result = CustomResult::default();
    assert_eq!(default_result, CustomResult::DumpAll(DumpAll::default()));
}

#[test]
fn test_custom_result_size() {
    let result = CustomResult::default();
    let result_size = result.size();
    let expected_size = DumpAll::default().size();
    assert_eq!(result_size, expected_size);
}

#[test]
fn test_custom_result_check() {
    let mut result = CustomResult::default();
    let res = result.check();
    assert!(res.is_ok());

    let mut dump_sessions = DumpSessions::default();
    let session = Session::default();
    dump_sessions.sessions.push((session.id, session.clone()));

    result = CustomResult::DumpSessions(dump_sessions);
    let res = result.check();
    assert!(res.is_err());

    let mut dump_items = DumpItems::default();
    dump_items.items.push((StoreKey::default(), StoreValue::default()));

    result = CustomResult::DumpItems(dump_items);
    let res = result.check();
    assert!(res.is_err());

    let mut dump_all = DumpAll::default();
    dump_all.sessions_count += 1;
    dump_all.items_count += 1;

    result = CustomResult::DumpAll(dump_all);
    let res = result.check();
    assert!(res.is_err());
}

#[test]
fn test_custom_result_serialize_json() {
    let result_a = CustomResult::default();
    
    let res = result_a.to_json();
    assert!(res.is_ok());
    let json = res.unwrap();

    let res = CustomResult::from_json(&json);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_custom_result_serialize_bytes() {
    let result_a = CustomResult::default();
    
    let res = result_a.to_bytes();
    assert!(res.is_ok());
    let bytes = res.unwrap();

    let res = CustomResult::from_bytes(&bytes);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_custom_result_serialize_hex() {
    let result_a = CustomResult::default();
    
    let res = result_a.to_hex();
    assert!(res.is_ok());
    let hex = res.unwrap();

    let res = CustomResult::from_hex(&hex);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}