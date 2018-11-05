use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;

use fixture::io::store::eval::{StoreEvalParams, DumpParams};

#[test]
fn test_eval_params_default() {
    let default_params = StoreEvalParams::default();
    assert_eq!(default_params, StoreEvalParams::Dump(DumpParams::default()));
}

#[test]
fn test_eval_params_size() {
    let params = StoreEvalParams::default();
    let params_size = params.size();
    assert_eq!(params_size, (DumpParams::default() as u8).size());

    let params = StoreEvalParams::Size;
    let params_size = params.size();
    assert_eq!(params_size, 0u8.size());
}

#[test]
fn test_eval_params_serialize_json() {
    let params_a = StoreEvalParams::default();
    
    let res = params_a.to_json();
    assert!(res.is_ok());
    let json = res.unwrap();

    let res = StoreEvalParams::from_json(&json);
    assert!(res.is_ok());
    let params_b = res.unwrap();

    assert_eq!(params_a, params_b);
}

#[test]
fn test_eval_params_serialize_bytes() {
    let params_a = StoreEvalParams::default();
    
    let res = params_a.to_bytes();
    assert!(res.is_ok());
    let bytes = res.unwrap();

    let res = StoreEvalParams::from_bytes(&bytes);
    assert!(res.is_ok());
    let params_b = res.unwrap();

    assert_eq!(params_a, params_b);
}

#[test]
fn test_eval_params_serialize_hex() {
    let params_a = StoreEvalParams::default();
    
    let res = params_a.to_hex();
    assert!(res.is_ok());
    let hex = res.unwrap();

    let res = StoreEvalParams::from_hex(&hex);
    assert!(res.is_ok());
    let params_b = res.unwrap();

    assert_eq!(params_a, params_b);
}