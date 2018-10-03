use mitrid_core::utils::{Timestamp, TimestampDiff};
use mitrid_core::base::Serializable;

#[test]
fn test_timestamp_add() {
    let tmsp_a = Timestamp::now().unwrap();
    let tmsp_diff = TimestampDiff::from_millis(1000);

    let tmsp_b = tmsp_a + tmsp_diff;
    assert_eq!(tmsp_b.as_millis(), tmsp_a.as_millis() + tmsp_diff.as_millis());
}

#[test]
fn test_timestamp_add_assign() {
    let mut tmsp = Timestamp::now().unwrap();
    let tmsp_diff = TimestampDiff::from_millis(1000);

    let tmsp_old = tmsp;
    tmsp += tmsp_diff;
    assert_eq!(tmsp.as_millis(), tmsp_old.as_millis() + tmsp_diff.as_millis());
}

#[test]
fn test_timestamp_sub() {
    let tmsp_a = Timestamp::now().unwrap();
    let tmsp_b = Timestamp::now().unwrap();

    let tmsp_diff = tmsp_a - tmsp_b;
    assert_eq!(tmsp_diff.as_millis(), tmsp_a.as_millis() - tmsp_b.as_millis());
}

#[test]
fn test_timestamp_mul() {
    let tmsp_a = Timestamp::now().unwrap();
    let multiplier = 1000;

    let tmsp_b = tmsp_a * multiplier;
    assert_eq!(tmsp_b.as_millis(), tmsp_a.as_millis() * multiplier);
}

#[test]
fn test_timestamp_mul_assign() {
    let mut tmsp = Timestamp::now().unwrap();
    let multiplier = 1000;

    let tmsp_old = tmsp;
    tmsp *= multiplier;
    assert_eq!(tmsp.as_millis(), tmsp_old.as_millis() * multiplier);
}

#[test]
fn test_timestamp_div() {
    let tmsp_a = Timestamp::now().unwrap();
    let tmsp_b = Timestamp::now().unwrap();

    let div = tmsp_a / tmsp_b;
    assert_eq!(div, tmsp_a.as_millis() / tmsp_b.as_millis());
}

#[test]
fn test_timestamp_rem() {
    let tmsp_a = Timestamp::now().unwrap();
    let tmsp_b = Timestamp::now().unwrap();

    let rem = tmsp_a % tmsp_b;
    assert_eq!(rem, tmsp_a.as_millis() % tmsp_b.as_millis());
}

#[test]
fn test_timestamp_serialize_json() {
    let tmsp_a = Timestamp::default();
    let res = tmsp_a.to_json();
    assert!(res.is_ok());

    let tmsp_a_json = res.unwrap();
    let res = Timestamp::from_json(&tmsp_a_json);
    assert!(res.is_ok());

    let tmsp_b = res.unwrap();
    assert_eq!(tmsp_a, tmsp_b);
}

#[test]
fn test_timestamp_serialize_bytes() {
    let tmsp_a = Timestamp::default();
    let res = tmsp_a.to_bytes();
    assert!(res.is_ok());

    let tmsp_a_bytes = res.unwrap();
    let res = Timestamp::from_bytes(&tmsp_a_bytes);
    assert!(res.is_ok());

    let tmsp_b = res.unwrap();
    assert_eq!(tmsp_a, tmsp_b);
}

#[test]
fn test_timestamp_serialize_hex() {
    let tmsp_a = Timestamp::default();
    let res = tmsp_a.to_hex();
    assert!(res.is_ok());

    let tmsp_a_hex = res.unwrap();
    let res = Timestamp::from_hex(&tmsp_a_hex);
    assert!(res.is_ok());

    let tmsp_b = res.unwrap();
    assert_eq!(tmsp_a, tmsp_b);
}