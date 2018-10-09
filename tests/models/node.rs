use mitrid_core::models::Node;
use mitrid_core::models::Meta;
use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;

use fixtures::Address;

#[test]
fn test_node_new() {
    let meta = Meta::default();
    let address = Address::new("address");
    let payload: String = "payload".into();
    
    let res = Node::new(&meta, &address, &payload);
    assert!(res.is_ok());
}

#[test]
fn test_node_check() {
    let meta = Meta::default();
    let address = Address::new("address");
    let payload: String = "payload".into();
    
    let mut node = Node::new(&meta, &address, &payload).unwrap();
    
    let res = node.check();
    assert!(res.is_ok());

    node.meta.size = 0;

    let res = node.check();
    assert!(res.is_err());
}

#[test]
fn test_node_size() {
    let node: Node<Address, String> = Node::default();
    let node_size = node.meta.size() +
                    node.address.size() +
                    node.payload.size();
    assert_eq!(node.size(), node_size)
}

#[test]
fn test_node_serialize_json() {
    let meta = Meta::default();
    let address = Address::new("address");
    let payload: String = "payload".into();
    
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
    let payload: String = "payload".into();
    
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
    let payload: String = "payload".into();
    
    let node_a = Node::new(&meta, &address, &payload).unwrap();

    let res = node_a.to_hex();
    assert!(res.is_ok());

    let node_a_hex = res.unwrap();
    let res = Node::from_hex(&node_a_hex);
    assert!(res.is_ok());

    let node_b = res.unwrap();
    assert_eq!(node_a, node_b);
}