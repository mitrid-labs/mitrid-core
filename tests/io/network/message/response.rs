use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::util::Version;
use mitrid_core::base::Meta;
use mitrid_core::io::network::Method;
use mitrid_core::io::network::Resource;
use mitrid_core::io::Storable;

use fixture::base::Payload;
use fixture::crypto::{Digest, Hasher};
use fixture::io::Session;
use fixture::io::Message;
use fixture::io::Response;
use fixture::io::store::*;

#[test]
fn test_response_new() {
    let valid_meta = Meta::default();

    let mut hasher = Hasher{};

    let mut message = Message::new()
                        .meta(&valid_meta)
                        .unwrap()
                        .session(&Session::default())
                        .unwrap()
                        .method(&Method::default())
                        .unwrap()
                        .resource(&Resource::default())
                        .unwrap()
                        .payload(&Payload::default())
                        .unwrap()
                        .finalize(&mut hasher)
                        .unwrap();

    let res = Response::new(&message);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    message.meta = invalid_meta;

    let res = Response::new(&message);
    assert!(res.is_err());
}

#[test]
fn test_response_size() {
    let meta = Meta::default();

    let mut hasher = Hasher{};

    let message = Message::new()
                    .meta(&meta)
                    .unwrap()
                    .session(&Session::default())
                    .unwrap()
                    .method(&Method::default())
                    .unwrap()
                    .resource(&Resource::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let response = Response::new(&message).unwrap();

    let meta_size = response.message.meta.get_size();
    let response_size = response.size();

    assert_eq!(meta_size, response_size);
}

#[test]
fn test_response_check() {
    let meta = Meta::default();

    let mut hasher = Hasher{};

    let message = Message::new()
                    .meta(&meta)
                    .unwrap()
                    .session(&Session::default())
                    .unwrap()
                    .method(&Method::default())
                    .unwrap()
                    .resource(&Resource::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let mut response = Response::new(&message).unwrap();

    let res = response.check();
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    response.message.meta = invalid_meta;

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

#[test]
fn test_response_store() {
    let meta = Meta::default();

    let mut hasher = Hasher{};

    let message = Message::new()
                    .meta(&meta)
                    .unwrap()
                    .session(&Session::default())
                    .unwrap()
                    .method(&Method::default())
                    .unwrap()
                    .resource(&Resource::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let response = Response::new(&message).unwrap();

    let mut store = Store::new();
    let res = response.store_create(&mut store);
    assert!(res.is_ok());

    let res = response.store_create(&mut store);
    assert!(res.is_err());

    let mut invalid_response = response.clone();

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    invalid_response.message.meta = invalid_meta;

    let res = invalid_response.store_create(&mut store);
    assert!(res.is_err());

    let res = Response::store_lookup(&mut store, &response.message.id);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let unknown_id = Digest::default();

    let res = Response::store_lookup(&mut store, &unknown_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Response::store_get(&mut store, &response.message.id);
    assert!(res.is_ok());

    let found_response = res.unwrap();
    assert_eq!(found_response, response);

    let res = Response::store_get(&mut store, &unknown_id);
    assert!(res.is_err());

    let mut from = Some(response.message.id.clone());
    let mut to = Some(response.message.id.clone());

    let res = Response::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Response::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(response.message.id.clone());

    let res = Response::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = None;
    to = Some(response.message.id.clone());

    let res = Response::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let mut from = Some(response.message.id.clone());
    let mut to = Some(response.message.id.clone());
    let mut count = None;
    let skip = 0;

    let res = Response::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_err());

    count = Some(0);

    let res = Response::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_err());

    from = None;
    to = None;
    count = None;

    let res = Response::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![response.clone()]);

    from = Some(response.message.id.clone());

    let res = Response::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![response.clone()]);

    from = None;
    to = Some(response.message.id.clone());

    let res = Response::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = response.store_delete(&mut store);
    assert!(res.is_ok());

    let res = response.store_delete(&mut store);
    assert!(res.is_err());

    let res = Response::store_lookup(&mut store, &response.message.id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Response::store_get(&mut store, &response.message.id);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Response::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let count = None;

    let res = Response::store_list(&mut store, to.clone(), from.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = response.store_upsert(&mut store);
    assert!(res.is_ok());

    let res = Response::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    let count = None;

    let res = Response::store_list(&mut store, to, from, count, skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![response.clone()]);
}