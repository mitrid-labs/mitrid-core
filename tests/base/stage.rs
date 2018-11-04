use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Stage;

#[test]
fn test_stage_parse() {
    let valid_stage_a = "testing";

    let res = Stage::parse(valid_stage_a);
    assert!(res.is_ok());

    let valid_stage_b = res.unwrap();
    assert_eq!(valid_stage_a, format!("{}", valid_stage_b));

    let invalid_stage = "test";

    let res = Stage::parse(invalid_stage);
    assert!(res.is_err());
}

#[test]
fn test_stage_size() {
    let stage = Stage::default();
    let stage_size = (stage as u8).size();
    assert_eq!(stage.size(), stage_size)
}

#[test]
fn test_stage_serialize_json() {
    let stage_a = Stage::default();
    let res = stage_a.to_json();
    assert!(res.is_ok());

    let stage_a_json = res.unwrap();
    let res = Stage::from_json(&stage_a_json);
    assert!(res.is_ok());

    let stage_b = res.unwrap();
    assert_eq!(stage_a, stage_b);
}

#[test]
fn test_stage_serialize_bytes() {
    let stage_a = Stage::default();
    let res = stage_a.to_bytes();
    assert!(res.is_ok());

    let stage_a_bytes = res.unwrap();
    let res = Stage::from_bytes(&stage_a_bytes);
    assert!(res.is_ok());

    let stage_b = res.unwrap();
    assert_eq!(stage_a, stage_b);
}

#[test]
fn test_stage_serialize_hex() {
    let stage_a = Stage::default();
    let res = stage_a.to_hex();
    assert!(res.is_ok());

    let stage_a_hex = res.unwrap();
    let res = Stage::from_hex(&stage_a_hex);
    assert!(res.is_ok());

    let stage_b = res.unwrap();
    assert_eq!(stage_a, stage_b);
}