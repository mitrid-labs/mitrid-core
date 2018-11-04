use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::utils::Version;
use mitrid_core::utils::Timestamp;
use mitrid_core::utils::Meta;
use mitrid_core::io::Permission;
use mitrid_core::io::network::Method;
use mitrid_core::io::network::Resource;

use fixtures::base::Payload;
use fixtures::crypto::SHA512HMAC;
use fixtures::io::Session;
use fixtures::io::Address;
use fixtures::io::Node;
use fixtures::io::network::message::*;

#[test]
fn test_message_meta() {
    let valid_meta = Meta::default();
    
    let res = Message::new().meta(&valid_meta);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let res = Message::new().meta(&invalid_meta);
    assert!(res.is_err())
}

#[test]
fn test_message_session() {
    let id = 0;
    let permission = Permission::default();
    let timestamp = Timestamp::now().unwrap();
    let session = Session::new(id, &permission, &timestamp, &()).unwrap();

    let res = Message::new().session(&session);
    assert!(res.is_ok())
}

#[test]
fn test_message_is_expired() {
    let id = 0;
    let permission = Permission::default();
    let expiration = Timestamp::now().unwrap();
    let session = Session::new(id, &permission, &expiration, &()).unwrap();

    let mut message = Message::new().session(&session).unwrap();
    let res = message.is_expired();
    assert!(res.is_ok());
    assert!(res.unwrap());

    message.session.expires_at *= 2;

    let res = message.is_expired();
    assert!(res.is_ok());
    assert!(!res.unwrap());
}

#[test]
fn test_message_sender() {
    let meta = Meta::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let mut node = Node::new(&meta, &address, &payload).unwrap();

    let message = Message::new();

    let res = message.clone().sender(&node);

    assert!(res.is_ok());

    let invalid_size = 0;
    node.meta.set_size(invalid_size);

    let res = message.sender(&node);

    assert!(res.is_err());
}

#[test]
fn test_message_receivers() {
    let meta = Meta::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let mut node = Node::new(&meta, &address, &payload).unwrap();

    let message = Message::new();

    let res = message.clone().receivers(&vec![node.clone()]);

    assert!(res.is_ok());

    let invalid_size = 0;
    node.meta.set_size(invalid_size);

    let res = message.receivers(&vec![node]);

    assert!(res.is_err());
}

#[test]
fn test_message_method() {
    let method_strs = vec!["ping",
                           "session",
                           "count",
                           "list",
                           "lookup",
                           "get",
                           "create",
                           "update",
                           "upsert",
                           "delete",
                           "custom"];

    for method_str in method_strs.iter() {
        let method = Method::parse(method_str).unwrap();
        let res = Message::new().method(&method);
        assert!(res.is_ok());
    }
}

#[test]
fn test_message_resource() {
    let resource_strs = vec!["none",
                             "session",
                             "node",
                             "coin",
                             "input",
                             "output",
                             "transaction",
                             "blocknode",
                             "block",
                             "blockgraph",
                             "custom",
                             "error"];

    for resource_str in resource_strs.iter() {
        let resource = Resource::parse(resource_str).unwrap();
        let res = Message::new().resource(&resource);
        assert!(res.is_ok());
    }
}

#[test]
fn test_message_is_error() {
    let resource_strs = vec!["none",
                             "session",
                             "node",
                             "coin",
                             "input",
                             "output",
                             "transaction",
                             "blocknode",
                             "block",
                             "blockgraph",
                             "custom",
                             "error"];

    for resource_str in resource_strs.iter() {
        let resource = Resource::parse(resource_str).unwrap();
        let message = Message::new().resource(&resource).unwrap();
        
        let is_error = message.is_error();

        if resource == Resource::Error {
            assert!(is_error);
        } else {
            assert!(!is_error);
        }
    }
}

#[test]
fn test_message_payload() {
    let payload = Payload::default();

    let res = Message::new().payload(&payload);
    assert!(res.is_ok())
}

#[test]
fn test_message_size() {
    let message = Message::new();

    let meta_size = message.meta.get_size();
    let message_size = message.size();

    assert_eq!(meta_size, message_size);
}

#[test]
fn test_message_check() {
    let valid_meta = Meta::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let node = Node::new(&valid_meta, &address, &payload).unwrap();

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
                        .unwrap();

    let res = message.check();
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    message.meta = invalid_meta;

    let res = message.check();
    assert!(res.is_err());
}

#[test]
fn test_digest() {
    let message = Message::new();

    let res = message.digest(&(), &message_digest_cb);
    assert!(res.is_ok());
}

#[test]
fn test_verify_digest() {
    let mut message = Message::new();

    message.id = message.digest(&(), &message_digest_cb).unwrap();
    
    let res = message.verify_digest(&(), &message_verify_digest_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_check_digest() {
    let mut message = Message::new();

    message.id = message.digest(&(), &message_digest_cb).unwrap();
    
    let res = message.check_digest(&(), &message_check_digest_cb);
    assert!(res.is_ok())
}

#[test]
fn test_message_commit() {
    let message = Message::new();

    let res = message.commit(&(), &message_commit_cb);
    assert!(res.is_ok());
}

#[test]
fn test_message_verify_commitment() {
    let message = Message::new();

    let commitment = message.commit(&(), &message_commit_cb).unwrap();
    
    let res = message.verify_commitment(&(), &commitment, &message_verify_commitment_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_message_check_commitment() {
    let message = Message::new();

    let commitment = message.commit(&(), &message_commit_cb).unwrap();
    
    let res = message.check_commitment(&(), &commitment, &message_check_commitment_cb);
    assert!(res.is_ok())
}

#[test]
fn test_message_authenticate() {
    let message = Message::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let res = message.authenticate(&key, &message_authenticate_cb);
    assert!(res.is_ok());
}

#[test]
fn test_message_verify_tag() {
    let message = Message::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let res = message.authenticate(&key, &message_authenticate_cb);
    assert!(res.is_ok());

    let tag = res.unwrap();
    
    let res = message.verify_tag(&key, &tag, &message_verify_tag_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_message_check_tag() {
    let message = Message::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let res = message.authenticate(&key, &message_authenticate_cb);
    assert!(res.is_ok());

    let tag = res.unwrap();
    
    let res = message.check_tag(&key, &tag, &message_check_tag_cb);
    assert!(res.is_ok())
}

#[test]
fn test_message_finalize() {
    let valid_meta = Meta::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let node = Node::new(&valid_meta, &address, &payload).unwrap();

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
                        .unwrap();

    let res = message.clone().finalize(&(), &message_digest_cb);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    message.meta = invalid_meta;

    let res = message.clone().finalize(&(), &message_digest_cb);
    assert!(res.is_err());
}

#[test]
fn test_message_json() {
    let message_a = Message::new();

    let res = message_a.to_json();
    assert!(res.is_ok());

    let message_json = res.unwrap();

    let res = Message::from_json(&message_json);
    assert!(res.is_ok());

    let message_b = res.unwrap();

    assert_eq!(message_a, message_b)
}

#[test]
fn test_message_bytes() {
    let message_a = Message::new();

    let res = message_a.to_bytes();
    assert!(res.is_ok());

    let message_bytes = res.unwrap();

    let res = Message::from_bytes(&message_bytes);
    assert!(res.is_ok());

    let message_b = res.unwrap();

    assert_eq!(message_a, message_b)
}

#[test]
fn test_message_hex() {
    let message_a = Message::new();

    let res = message_a.to_hex();
    assert!(res.is_ok());

    let message_hex = res.unwrap();

    let res = Message::from_hex(&message_hex);
    assert!(res.is_ok());

    let message_b = res.unwrap();

    assert_eq!(message_a, message_b)
}

/*
#[test]
fn test_message_store() {
    let valid_meta = Meta::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let node = Node::new(&valid_meta, &address, &payload).unwrap();

    let message = Message::new()
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
                    .finalize(&(), &message_digest_cb)
                    .unwrap();

    let mut store = Store::new();
    let res = message.store_create(&mut store, &());
    assert!(res.is_ok());

    let res = message.store_create(&mut store, &());
    assert!(res.is_err());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let mut invalid_message = message.clone();

    invalid_message.meta = invalid_meta;

    let res = invalid_message.store_create(&mut store, &());
    assert!(res.is_err());

    let res = Message::store_lookup(&mut store, &(), &message.id);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let unknown_id = Digest::default();

    let res = Message::store_lookup(&mut store, &(), &unknown_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Message::store_get(&mut store, &(), &message.id);
    assert!(res.is_ok());

    let found_message = res.unwrap();
    assert_eq!(found_message, message);

    let res = Message::store_get(&mut store, &(), &unknown_id);
    assert!(res.is_err());

    let mut from = Some(message.id.clone());
    let mut to = Some(message.id.clone());

    let res = Message::store_count(&mut store, &(), &from, &to);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Message::store_count(&mut store, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(message.id.clone());

    let res = Message::store_count(&mut store, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = None;
    to = Some(message.id.clone());

    let res = Message::store_count(&mut store, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let mut from = Some(message.id.clone());
    let mut to = Some(message.id.clone());
    let mut count = None;

    let res = Message::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_err());

    count = Some(0);

    let res = Message::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_err());

    from = None;
    to = None;
    count = None;

    let res = Message::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![message.clone()]);

    from = Some(message.id.clone());

    let res = Message::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![message.clone()]);

    from = None;
    to = Some(message.id.clone());

    let res = Message::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = message.store_delete(&mut store, &());
    assert!(res.is_ok());

    let res = message.store_delete(&mut store, &());
    assert!(res.is_err());

    let res = Message::store_lookup(&mut store, &(), &message.id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Message::store_get(&mut store, &(), &message.id);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Message::store_count(&mut store, &(), &to, &from);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let count = None;

    let res = Message::store_list(&mut store, &(), &to, &from, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = message.store_upsert(&mut store, &());
    assert!(res.is_ok());

    let res = Message::store_count(&mut store, &(), &to, &from);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    let count = None;

    let res = Message::store_list(&mut store, &(), &to, &from, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![message.clone()]);
}
*/