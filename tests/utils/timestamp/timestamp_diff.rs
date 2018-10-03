use std::time::{SystemTime, UNIX_EPOCH};

use mitrid_core::utils::timestamp::TimestampDiff;
use mitrid_core::base::Serializable;

#[test]
fn test_timestamp_diff_duration() {
    let duration_a = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let tmsp_diff = TimestampDiff::from_duration(duration_a);
    
    let duration_b = tmsp_diff.as_duration();

    let secs_a = duration_a.as_secs();
    let millis_a = duration_a.subsec_millis() as u64;

    let secs_b = duration_b.as_secs();
    let millis_b = duration_b.subsec_millis() as u64;

    assert_eq!(secs_a + millis_a, secs_b + millis_b);
}

#[test]
fn test_timestamp_diff_millis() {
    let millis_a = 1000;
    let timestamp_diff = TimestampDiff::from_millis(millis_a);

    let millis_b = timestamp_diff.as_millis();
    assert_eq!(millis_a, millis_b);
}

#[test]
fn test_timestamp_diff_secs() {
    let secs_a = 1;
    let timestamp_diff = TimestampDiff::from_secs(secs_a);

    let secs_b = timestamp_diff.as_secs();
    assert_eq!(secs_a, secs_b);
}

#[test]
fn test_timestamp_diff_serialize_json() {
    let tmsp_diff_a = TimestampDiff::default();
    let res = tmsp_diff_a.to_json();
    assert!(res.is_ok());

    let tmsp_diff_a_json = res.unwrap();
    let res = TimestampDiff::from_json(&tmsp_diff_a_json);
    assert!(res.is_ok());

    let tmsp_diff_b = res.unwrap();
    assert_eq!(tmsp_diff_a, tmsp_diff_b);
}

#[test]
fn test_timestamp_diff_serialize_bytes() {
    let tmsp_diff_a = TimestampDiff::default();
    let res = tmsp_diff_a.to_bytes();
    assert!(res.is_ok());

    let tmsp_diff_a_bytes = res.unwrap();
    let res = TimestampDiff::from_bytes(&tmsp_diff_a_bytes);
    assert!(res.is_ok());

    let tmsp_diff_b = res.unwrap();
    assert_eq!(tmsp_diff_a, tmsp_diff_b);
}

#[test]
fn test_timestamp_diff_serialize_hex() {
    let tmsp_diff_a = TimestampDiff::default();
    let res = tmsp_diff_a.to_hex();
    assert!(res.is_ok());

    let tmsp_diff_a_hex = res.unwrap();
    let res = TimestampDiff::from_hex(&tmsp_diff_a_hex);
    assert!(res.is_ok());

    let tmsp_diff_b = res.unwrap();
    assert_eq!(tmsp_diff_a, tmsp_diff_b);
}