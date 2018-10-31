use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;

use fixtures::io::store::{StoreKey, StoreValue};
use fixtures::io::store::custom::DumpItems;

#[test]
fn test_dump_items_new() {
    let items = vec![(StoreKey::default(), StoreValue::default())];
    let result = DumpItems::new(&items);

    assert_eq!(result.count, result.items.len() as u64);
}

#[test]
fn test_dump_items_default() {
    let default_result = DumpItems::default();
    let expected_result = DumpItems {
        count: 0,
        items: Vec::new(),
    };
    assert_eq!(default_result, expected_result);
}

#[test]
fn test_dump_items_size() {
    let result = DumpItems::default();
    let result_size = result.size();
    let expected_size = result.count.size() + result.items.size();
    assert_eq!(result_size, expected_size);
}

#[test]
fn test_dump_items_check() {
    let mut result = DumpItems::default();
    let res = result.check();
    assert!(res.is_ok());

    result.count += 1;
    let res = result.check();
    assert!(res.is_err());

    result.items.push((StoreKey::default(), StoreValue::default()));
    let res = result.check();
    assert!(res.is_ok());
}

#[test]
fn test_dump_items_serialize_json() {
    let result_a = DumpItems::default();
    
    let res = result_a.to_json();
    assert!(res.is_ok());
    let json = res.unwrap();

    let res = DumpItems::from_json(&json);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_dump_items_serialize_bytes() {
    let result_a = DumpItems::default();
    
    let res = result_a.to_bytes();
    assert!(res.is_ok());
    let bytes = res.unwrap();

    let res = DumpItems::from_bytes(&bytes);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_dump_items_serialize_hex() {
    let result_a = DumpItems::default();
    
    let res = result_a.to_hex();
    assert!(res.is_ok());
    let hex = res.unwrap();

    let res = DumpItems::from_hex(&hex);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}