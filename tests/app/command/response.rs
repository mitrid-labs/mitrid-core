use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Meta;

use fixtures::base::Payload;
use fixtures::app::command::Address;
use fixtures::app::command::Response;

#[test]
fn test_response_new() {
    let address = Address::default();
    let meta = Meta::default();
    let result = Some(Payload::default());
    let mut error = None;

    let res = Response::new(&address, &meta, &result, &error);
    assert!(res.is_ok());

    error = Some(format!("an error"));

    let res = Response::new(&address, &meta, &result, &error);
    assert!(res.is_err());
}

#[test]
fn test_response_size() {
    let address = Address::default();
    let meta = Meta::default();
    let result = Some(Payload::default());
    let error = None;

    let response = Response::new(&address, &meta, &result, &error).unwrap();

    let meta_size = response.meta.get_size();
    let response_size = response.size();

    assert_eq!(meta_size, response_size);
}

#[test]
fn test_response_check() {
    let address = Address::default();
    let meta = Meta::default();
    let result = Some(Payload::default());
    let error = None;

    let mut response = Response::new(&address, &meta, &result, &error).unwrap();

    let res = response.check();
    assert!(res.is_ok());

    response.error = Some(format!("an error"));

    let res = response.check();
    assert!(res.is_err());
}

#[test]
fn test_response_json() {
    let response_a = Response::default();

    let res = response_a.to_json();
    assert!(res.is_ok());

    let response_json = res.unwrap();

    let res = Response::from_json(&response_json);
    assert!(res.is_ok());

    let response_b = res.unwrap();

    assert_eq!(response_a, response_b)
}

#[test]
fn test_response_bytes() {
    let response_a = Response::default();

    let res = response_a.to_bytes();
    assert!(res.is_ok());

    let response_bytes = res.unwrap();

    let res = Response::from_bytes(&response_bytes);
    assert!(res.is_ok());

    let response_b = res.unwrap();

    assert_eq!(response_a, response_b)
}

#[test]
fn test_response_hex() {
    let response_a = Response::default();

    let res = response_a.to_hex();
    assert!(res.is_ok());

    let response_hex = res.unwrap();

    let res = Response::from_hex(&response_hex);
    assert!(res.is_ok());

    let response_b = res.unwrap();

    assert_eq!(response_a, response_b)
}