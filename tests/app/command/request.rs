use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::util::Version;
use mitrid_core::base::Meta;

use fixture::base::Payload;
use fixture::app::command::Address;
use fixture::app::command::Request;

#[test]
fn test_request_new() {
    let address = Address::default();
    let mut meta = Meta::default();
    let params = Payload::default();

    let res = Request::new(&address, &meta, &params);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();
    meta.version = invalid_version;

    let res = Request::new(&address, &meta, &params);
    assert!(res.is_err());
}

#[test]
fn test_request_size() {
    let address = Address::default();
    let meta = Meta::default();
    let params = Payload::default();

    let request = Request::new(&address, &meta, &params).unwrap();

    let meta_size = request.meta.get_size();
    let request_size = request.size();

    assert_eq!(meta_size, request_size);
}

#[test]
fn test_request_check() {
    let address = Address::default();
    let meta = Meta::default();
    let params = Payload::default();

    let mut request = Request::new(&address, &meta, &params).unwrap();

    let res = request.check();
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();
    request.meta.version = invalid_version;

    let res = request.check();
    assert!(res.is_err());
}

#[test]
fn test_request_json() {
    let request_a = Request::default();

    let res = request_a.to_json();
    assert!(res.is_ok());

    let request_json = res.unwrap();

    let res = Request::from_json(&request_json);
    assert!(res.is_ok());

    let request_b = res.unwrap();

    assert_eq!(request_a, request_b)
}

#[test]
fn test_request_bytes() {
    let request_a = Request::default();

    let res = request_a.to_bytes();
    assert!(res.is_ok());

    let request_bytes = res.unwrap();

    let res = Request::from_bytes(&request_bytes);
    assert!(res.is_ok());

    let request_b = res.unwrap();

    assert_eq!(request_a, request_b)
}

#[test]
fn test_request_hex() {
    let request_a = Request::default();

    let res = request_a.to_hex();
    assert!(res.is_ok());

    let request_hex = res.unwrap();

    let res = Request::from_hex(&request_hex);
    assert!(res.is_ok());

    let request_b = res.unwrap();

    assert_eq!(request_a, request_b)
}