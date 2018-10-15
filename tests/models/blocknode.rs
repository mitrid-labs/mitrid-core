use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::utils::Version;
use mitrid_core::models::Meta;

use fixtures::crypto::Digest;
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
fn test_blocknode_block_data() {
    let block_id = Digest::default();
    let block_height = 0;

    let res = BlockNode::new().block_data(&block_id, block_height);
    assert!(res.is_ok());
}

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
fn test_blocknode_commit() {
    let bn = BlockNode::new();

    let res = bn.commit(&(), &blocknode_commit_cb);
    assert!(res.is_ok());
}

#[test]
fn test_blocknode_verify_commitment() {
    let bn = BlockNode::new();

    let commitment = bn.commit(&(), &blocknode_commit_cb).unwrap();
    
    let res = bn.verify_commitment(&(), &commitment, &blocknode_verify_commitment_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_blocknode_check_commitment() {
    let bn = BlockNode::new();

    let commitment = bn.commit(&(), &blocknode_commit_cb).unwrap();
    
    let res = bn.check_commitment(&(), &commitment, &blocknode_check_commitment_cb);
    assert!(res.is_ok())
}

#[test]
fn test_blocknode_finalize() {
    let block_id = Digest::default();
    let block_height = 0;

    let mut blocknode = BlockNode::new()
                            .meta(&Meta::default())
                            .unwrap()
                            .block_data(&block_id, block_height)
                            .unwrap();

    let res = blocknode.clone().finalize(&(), &blocknode_digest_cb);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;
    blocknode.meta = invalid_meta;

    let res = blocknode.finalize(&(), &blocknode_digest_cb);
    assert!(res.is_err());
}

#[test]
fn test_blocknode_check() {
    let block_id = Digest::default();
    let block_height = 0;

    let mut blocknode = BlockNode::new()
                            .meta(&Meta::default())
                            .unwrap()
                            .block_data(&block_id, block_height)
                            .unwrap()
                            .finalize(&(), &blocknode_digest_cb)
                            .unwrap();

    let res = blocknode.check();
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let invalid_meta = Meta::default();
    blocknode.meta = invalid_meta;

    let res = blocknode.check();
    assert!(res.is_err());
}

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