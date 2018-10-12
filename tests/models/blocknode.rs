//use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::utils::Version;
use mitrid_core::models::Meta;

//use fixtures::models::Amount;
//use fixtures::models::Payload;
use fixtures::models::blocknode::*;

#[test]
fn test_blocknode_meta() {
    let valid_meta = Meta::default();
    
    let res = BlockNode::new().meta(&valid_meta);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let res = BlockNode::new().meta(&invalid_meta);
    assert!(res.is_err())
}

#[test]
fn test_blocknode_block_data() {}

#[test]
fn test_blocknode_digest() {
    let blocknode = BlockNode::new();

    let res = blocknode.digest(&(), &blocknode_digest_cb);
    assert!(res.is_ok());
}

#[test]
fn test_blocknode_verify_digest() {
    let mut blocknode = BlockNode::new();

    blocknode.id = blocknode.digest(&(), &blocknode_digest_cb).unwrap();
    
    let res = blocknode.verify_digest(&(), &blocknode_verify_digest_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_blocknode_check_digest() {
    let mut blocknode = BlockNode::new();

    blocknode.id = blocknode.digest(&(), &blocknode_digest_cb).unwrap();
    
    let res = blocknode.check_digest(&(), &blocknode_check_digest_cb);
    assert!(res.is_ok())
}

#[test]
fn test_blocknode_finalize() {}

#[test]
fn test_blocknode_check() {}

#[test]
fn test_blocknode_size() {
    let blocknode = BlockNode::new();

    let meta_size = blocknode.meta.get_size();
    let blocknode_size = blocknode.size();

    assert_eq!(meta_size, blocknode_size);
}

#[test]
fn test_blocknode_json() {
    let blocknode_a = BlockNode::new();

    let res = blocknode_a.to_json();
    assert!(res.is_ok());

    let blocknode_json = res.unwrap();

    let res = BlockNode::from_json(&blocknode_json);
    assert!(res.is_ok());

    let blocknode_b = res.unwrap();

    assert_eq!(blocknode_a, blocknode_b);
}

#[test]
fn test_blocknode_bytes() {
    let blocknode_a = BlockNode::new();

    let res = blocknode_a.to_bytes();
    assert!(res.is_ok());

    let blocknode_bytes = res.unwrap();

    let res = BlockNode::from_bytes(&blocknode_bytes);
    assert!(res.is_ok());

    let blocknode_b = res.unwrap();

    assert_eq!(blocknode_a, blocknode_b);
}

#[test]
fn test_blocknode_hex() {
    let blocknode_a = BlockNode::new();

    let res = blocknode_a.to_hex();
    assert!(res.is_ok());

    let blocknode_hex = res.unwrap();

    let res = BlockNode::from_hex(&blocknode_hex);
    assert!(res.is_ok());

    let blocknode_b = res.unwrap();

    assert_eq!(blocknode_a, blocknode_b);
}