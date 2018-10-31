use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;

use fixtures::io::store::custom::DumpParams;

#[test]
fn test_dump_params_parse() {
    let valid_str_params = vec!["sessions", "items", "all"];
    let invalid_str_param = "size";

    for str_param in valid_str_params {
        let res = DumpParams::parse(str_param);
        assert!(res.is_ok());

        let params = res.unwrap();
        assert_eq!(format!("{}", params), str_param.to_string());
    }

    let res = DumpParams::parse(invalid_str_param);
    assert!(res.is_err());
}

#[test]
fn test_dump_params_default() {
    let default_params = DumpParams::default();
    assert_eq!(default_params, DumpParams::All)
}

#[test]
fn test_dump_params_size() {
    let params = DumpParams::default();
    let params_size = params.size();
    assert_eq!(params_size, (params as u8).size());
}

#[test]
fn test_dump_params_serialize_json() {
    let params_a = DumpParams::default();
    
    let res = params_a.to_json();
    assert!(res.is_ok());
    let json = res.unwrap();

    let res = DumpParams::from_json(&json);
    assert!(res.is_ok());
    let params_b = res.unwrap();

    assert_eq!(params_a, params_b);
}

#[test]
fn test_dump_params_serialize_bytes() {
    let params_a = DumpParams::default();
    
    let res = params_a.to_bytes();
    assert!(res.is_ok());
    let bytes = res.unwrap();

    let res = DumpParams::from_bytes(&bytes);
    assert!(res.is_ok());
    let params_b = res.unwrap();

    assert_eq!(params_a, params_b);
}

#[test]
fn test_dump_params_serialize_hex() {
    let params_a = DumpParams::default();
    
    let res = params_a.to_hex();
    assert!(res.is_ok());
    let hex = res.unwrap();

    let res = DumpParams::from_hex(&hex);
    assert!(res.is_ok());
    let params_b = res.unwrap();

    assert_eq!(params_a, params_b);
}