use mitrid_core::utils::Meta;
use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::io::Storable;

use fixtures::base::Payload;
use fixtures::io::Address;
use fixtures::io::Node;
use fixtures::io::store::*;

#[test]
fn test_node_new() {
    let meta = Meta::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let res = Node::new(&meta, &address, &payload);
    assert!(res.is_ok());
}

#[test]
fn test_node_check() {
    let meta = Meta::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let mut node = Node::new(&meta, &address, &payload).unwrap();
    
    let res = node.check();
    assert!(res.is_ok());

    let invalid_size = 0;
    node.meta.set_size(invalid_size);

    let res = node.check();
    assert!(res.is_err());
}

#[test]
fn test_node_size() {
    let node = Node::default();
    let node_size = node.meta.size() +
                    node.address.size() +
                    node.payload.size();
    assert_eq!(node.size(), node_size)
}

#[test]
fn test_node_serialize_json() {
    let meta = Meta::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let node_a = Node::new(&meta, &address, &payload).unwrap();

    let res = node_a.to_json();
    assert!(res.is_ok());

    let node_a_json = res.unwrap();
    let res = Node::from_json(&node_a_json);
    assert!(res.is_ok());

    let node_b = res.unwrap();
    assert_eq!(node_a, node_b);
}

#[test]
fn test_node_serialize_bytes() {
    let meta = Meta::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let node_a = Node::new(&meta, &address, &payload).unwrap();

    let res = node_a.to_bytes();
    assert!(res.is_ok());

    let node_a_bytes = res.unwrap();
    let res = Node::from_bytes(&node_a_bytes);
    assert!(res.is_ok());

    let node_b = res.unwrap();
    assert_eq!(node_a, node_b);
}

#[test]
fn test_node_serialize_hex() {
    let meta = Meta::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let node_a = Node::new(&meta, &address, &payload).unwrap();

    let res = node_a.to_hex();
    assert!(res.is_ok());

    let node_a_hex = res.unwrap();
    let res = Node::from_hex(&node_a_hex);
    assert!(res.is_ok());

    let node_b = res.unwrap();
    assert_eq!(node_a, node_b);
}

#[test]
fn test_node_store() {
    let meta = Meta::default();
    let address = Address::new("address");
    let payload = Payload::new("payload");
    
    let mut node = Node::new(&meta, &address, &payload).unwrap();

    let mut store = Store::new();
    let res = node.store_create(&mut store, &());
    assert!(res.is_ok());

    let res = node.store_create(&mut store, &());
    assert!(res.is_err());

    let invalid_size = 0;

    let mut invalid_node = node.clone();

    invalid_node.meta.set_size(invalid_size);

    let res = invalid_node.store_create(&mut store, &());
    assert!(res.is_err());

    let res = Node::store_lookup(&mut store, &(), &node.address);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let unknown_address = Address::default();

    let res = Node::store_lookup(&mut store, &(), &unknown_address);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Node::store_get(&mut store, &(), &node.address);
    assert!(res.is_ok());

    let found_node = res.unwrap();
    assert_eq!(found_node, node);

    let res = Node::store_get(&mut store, &(), &unknown_address);
    assert!(res.is_err());

    let mut from = Some(node.address.clone());
    let mut to = Some(node.address.clone());

    let res = Node::store_count(&mut store, &(), &from, &to);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Node::store_count(&mut store, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(node.address.clone());

    let res = Node::store_count(&mut store, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = None;
    to = Some(node.address.clone());

    let res = Node::store_count(&mut store, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let mut from = Some(node.address.clone());
    let mut to = Some(node.address.clone());
    let mut count = None;

    let res = Node::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_err());

    count = Some(0);

    let res = Node::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_err());

    from = None;
    to = None;
    count = None;

    let res = Node::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![node.clone()]);

    from = Some(node.address.clone());

    let res = Node::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![node.clone()]);

    from = None;
    to = Some(node.address.clone());

    let res = Node::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    node.payload = Payload::new("An other one");
    
    let res = node.store_update(&mut store, &());
    assert!(res.is_err());

    node.update_size();
    
    let res = node.store_update(&mut store, &());
    assert!(res.is_ok());

    node.payload = Payload::new("Again");
    node.update_size();
    
    let res = node.store_update(&mut store, &());
    assert!(res.is_ok());

    let res = Node::store_get(&mut store, &(), &node.address);
    assert!(res.is_ok());

    let found_node = res.unwrap();
    assert_eq!(found_node, node);

    let res = node.store_delete(&mut store, &());
    assert!(res.is_ok());

    let res = node.store_delete(&mut store, &());
    assert!(res.is_err());

    let res = Node::store_lookup(&mut store, &(), &node.address);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Node::store_get(&mut store, &(), &node.address);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Node::store_count(&mut store, &(), &to, &from);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let count = None;

    let res = Node::store_list(&mut store, &(), &to, &from, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = node.store_upsert(&mut store, &());
    assert!(res.is_ok());

    let res = Node::store_count(&mut store, &(), &to, &from);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    let count = None;

    let res = Node::store_list(&mut store, &(), &to, &from, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![node.clone()]);
}