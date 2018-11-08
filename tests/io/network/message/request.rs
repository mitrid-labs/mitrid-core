use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Meta;
use mitrid_core::io::network::Method;
use mitrid_core::io::network::Resource;
use mitrid_core::io::Storable;

use fixture::base::Payload;
use fixture::crypto::{Digest, Hasher};
use fixture::io::Session;
use fixture::io::Address;
use fixture::io::Node;
use fixture::io::Message;
use fixture::io::Request;
use fixture::io::store::*;

#[test]
fn test_request_new() {
    let valid_meta = Meta::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let node = Node::new(&valid_meta, &address, &payload).unwrap();

    let mut hasher = Hasher{};

    let mut message = Message::new()
                        .meta(&valid_meta)
                        .unwrap()
                        .session(&Session::default())
                        .unwrap()
                        .sender(&node)
                        .unwrap()
                        .receivers(&vec![node.clone()])
                        .unwrap()
                        .method(&Method::default())
                        .unwrap()
                        .resource(&Resource::default())
                        .unwrap()
                        .payload(&Payload::default())
                        .unwrap()
                        .finalize(&mut hasher)
                        .unwrap();

    let res = Request::new(&message);
    assert!(res.is_ok());

    message.resource = Resource::Error;

    let res = Request::new(&message);
    assert!(res.is_err());
}

#[test]
fn test_request_size() {
    let meta = Meta::default();
    let session = Session::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let node = Node::new(&meta, &address, &payload).unwrap();
    let method = Method::default();
    let resource = Resource::default();
    let payload = Payload::default();

    let mut hasher = Hasher{};

    let message = Message::new()
                    .meta(&meta)
                    .unwrap()
                    .session(&session)
                    .unwrap()
                    .sender(&node)
                    .unwrap()
                    .receivers(&vec![node.clone()])
                    .unwrap()
                    .method(&method)
                    .unwrap()
                    .resource(&resource)
                    .unwrap()
                    .payload(&payload)
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let request = Request::new(&message).unwrap();

    let meta_size = request.message.meta.get_size();
    let request_size = request.size();

    assert_eq!(meta_size, request_size);
}

#[test]
fn test_request_check() {
    let meta = Meta::default();
    let session = Session::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let node = Node::new(&meta, &address, &payload).unwrap();
    let method = Method::default();
    let resource = Resource::default();
    let payload = Payload::default();

    let mut hasher = Hasher{};

    let message = Message::new()
                    .meta(&meta)
                    .unwrap()
                    .session(&session)
                    .unwrap()
                    .sender(&node)
                    .unwrap()
                    .receivers(&vec![node.clone()])
                    .unwrap()
                    .method(&method)
                    .unwrap()
                    .resource(&resource)
                    .unwrap()
                    .payload(&payload)
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let mut request = Request::new(&message).unwrap();

    let res = request.check();
    assert!(res.is_ok());

    request.message.resource = Resource::Error;

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

#[test]
fn test_request_store() {
    let meta = Meta::default();
    let session = Session::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let node = Node::new(&meta, &address, &payload).unwrap();
    let method = Method::default();
    let resource = Resource::default();
    let payload = Payload::default();

    let mut hasher = Hasher{};

    let message = Message::new()
                    .meta(&meta)
                    .unwrap()
                    .session(&session)
                    .unwrap()
                    .sender(&node)
                    .unwrap()
                    .receivers(&vec![node.clone()])
                    .unwrap()
                    .method(&method)
                    .unwrap()
                    .resource(&resource)
                    .unwrap()
                    .payload(&payload)
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let request = Request::new(&message).unwrap();

    let mut store = Store::new();
    let res = request.store_create(&mut store);
    assert!(res.is_ok());

    let res = request.store_create(&mut store);
    assert!(res.is_err());

    let mut invalid_request = request.clone();

    invalid_request.message.resource = Resource::Error;

    let res = invalid_request.store_create(&mut store);
    assert!(res.is_err());

    let res = Request::store_lookup(&mut store, &request.message.id);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let unknown_id = Digest::default();

    let res = Request::store_lookup(&mut store, &unknown_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Request::store_get(&mut store, &request.message.id);
    assert!(res.is_ok());

    let found_request = res.unwrap();
    assert_eq!(found_request, request);

    let res = Request::store_get(&mut store, &unknown_id);
    assert!(res.is_err());

    let mut from = Some(request.message.id.clone());
    let mut to = Some(request.message.id.clone());
    let skip = 0;

    let res = Request::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Request::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(request.message.id.clone());

    let res = Request::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = None;
    to = Some(request.message.id.clone());

    let res = Request::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let mut from = Some(request.message.id.clone());
    let mut to = Some(request.message.id.clone());
    let mut count = None;

    let res = Request::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_err());

    count = Some(0);

    let res = Request::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_err());

    from = None;
    to = None;
    count = None;

    let res = Request::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![request.clone()]);

    from = Some(request.message.id.clone());

    let res = Request::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![request.clone()]);

    from = None;
    to = Some(request.message.id.clone());

    let res = Request::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = request.store_delete(&mut store);
    assert!(res.is_ok());

    let res = request.store_delete(&mut store);
    assert!(res.is_err());

    let res = Request::store_lookup(&mut store, &request.message.id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Request::store_get(&mut store, &request.message.id);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Request::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let count = None;

    let res = Request::store_list(&mut store, to.clone(), from.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = request.store_upsert(&mut store);
    assert!(res.is_ok());

    let res = Request::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    let count = None;

    let res = Request::store_list(&mut store, to, from, count, skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![request.clone()]);
}