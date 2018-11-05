use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::util::Version;
use mitrid_core::util::Timestamp;
use mitrid_core::base::Meta;
use mitrid_core::io::Permission;
use mitrid_core::io::network::Method;
use mitrid_core::io::network::Resource;

use fixture::base::eval::*;
use fixture::base::Payload;
use fixture::crypto::Hasher;
use fixture::io::Session;
use fixture::io::Address;
use fixture::io::Node;
use fixture::io::message::*;

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
                           "eval",
                           "evalmut"];

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
                             "evalparams",
                             "evalresult",
                             "evalmutparams",
                             "evalmutresult",
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
                             "evalparams",
                             "evalresult",
                             "evalmutparams",
                             "evalmutresult",
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
fn test_message_eval() {
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
                        .payload(&payload)
                        .unwrap()
                        .finalize(&mut hasher)
                        .unwrap();

    let mut evaluator = MessageEvaluator{};

    let res = message.eval(&PayloadEvalParams::Const, &evaluator);
    assert!(res.is_ok());

    let const_res = res.unwrap();
    assert_eq!(const_res, PayloadEvalResult::Const(payload.to_string()));

    let res = message.eval_mut(&PayloadEvalMutParams::ToUppercase, &mut evaluator);
    assert!(res.is_ok());

    let to_uppercase_res = res.unwrap();

    let uppsercase_payload = payload.to_string().to_uppercase();
    assert_eq!(to_uppercase_res, PayloadEvalMutResult::ToUppercase(uppsercase_payload.clone()));
    assert_eq!(message.payload.to_string(), uppsercase_payload)
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
                        .payload(&payload)
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

    let mut hasher = Hasher{};
    let res = message.digest(&mut hasher);
    assert!(res.is_ok());
}

#[test]
fn test_verify_digest() {
    let mut message = Message::new();

    let mut hasher = Hasher{};

    message.id = message.digest(&mut hasher).unwrap();
    
    let res = message.verify_digest(&mut hasher);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_check_digest() {
    let mut message = Message::new();

    let mut hasher = Hasher{};

    message.id = message.digest(&mut hasher).unwrap();
    
    let res = message.check_digest(&mut hasher);
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
                        .payload(&payload)
                        .unwrap();

    let mut hasher = Hasher{};

    let res = message.clone().finalize(&mut hasher);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    message.meta = invalid_meta;

    let res = message.clone().finalize(&mut hasher);
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