use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::utils::Version;
use mitrid_core::base::Meta;
use mitrid_core::io::Storable;

use fixtures::crypto::Digest;
use fixtures::crypto::SHA512HMAC;
use fixtures::models::blocknode::*;
use fixtures::io::store::*;

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
fn test_blocknode_authenticate() {
    let bn = BlockNode::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let res = bn.authenticate(&key, &blocknode_authenticate_cb);
    assert!(res.is_ok());
}

#[test]
fn test_blocknode_verify_tag() {
    let bn = BlockNode::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let tag = bn.authenticate(&key, &blocknode_authenticate_cb).unwrap();
    
    let res = bn.verify_tag(&key, &tag, &blocknode_verify_tag_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_blocknode_check_tag() {
    let bn = BlockNode::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let tag = bn.authenticate(&key, &blocknode_authenticate_cb).unwrap();
    
    let res = bn.check_tag(&key, &tag, &blocknode_check_tag_cb);
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

#[test]
fn test_blocknode_store() {
    let block_id = Digest::default();
    let block_height = 0;

    let blocknode = BlockNode::new()
                        .meta(&Meta::default())
                        .unwrap()
                        .block_data(&block_id, block_height)
                        .unwrap()
                        .finalize(&(), &blocknode_digest_cb)
                        .unwrap();

    let mut store = Store::new();
    let res = blocknode.store_create(&mut store);
    assert!(res.is_ok());

    let res = blocknode.store_create(&mut store);
    assert!(res.is_err());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let invalid_meta = Meta::default();

    let mut invalid_blocknode = blocknode.clone();
    invalid_blocknode.meta = invalid_meta;

    let res = invalid_blocknode.store_create(&mut store);
    assert!(res.is_err());

    let res = BlockNode::store_lookup(&mut store, &blocknode.id);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let unknown_id = Digest::default();

    let res = BlockNode::store_lookup(&mut store, &unknown_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = BlockNode::store_get(&mut store, &blocknode.id);
    assert!(res.is_ok());

    let found_blocknode = res.unwrap();
    assert_eq!(found_blocknode, blocknode);

    let res = BlockNode::store_get(&mut store, &unknown_id);
    assert!(res.is_err());

    let mut from = Some(blocknode.id.clone());
    let mut to = Some(blocknode.id.clone());

    let res = BlockNode::store_count(&mut store, &from, &to);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = BlockNode::store_count(&mut store, &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(blocknode.id.clone());

    let res = BlockNode::store_count(&mut store, &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = None;
    to = Some(blocknode.id.clone());

    let res = BlockNode::store_count(&mut store, &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let mut from = Some(blocknode.id.clone());
    let mut to = Some(blocknode.id.clone());
    let mut count = None;

    let res = BlockNode::store_list(&mut store, &from, &to, &count);
    assert!(res.is_err());

    count = Some(0);

    let res = BlockNode::store_list(&mut store, &from, &to, &count);
    assert!(res.is_err());

    from = None;
    to = None;
    count = None;

    let res = BlockNode::store_list(&mut store, &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![blocknode.clone()]);

    from = Some(blocknode.id.clone());

    let res = BlockNode::store_list(&mut store, &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![blocknode.clone()]);

    from = None;
    to = Some(blocknode.id.clone());

    let res = BlockNode::store_list(&mut store, &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = blocknode.store_delete(&mut store);
    assert!(res.is_ok());

    let res = blocknode.store_delete(&mut store);
    assert!(res.is_err());

    let res = BlockNode::store_lookup(&mut store, &blocknode.id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = BlockNode::store_get(&mut store, &blocknode.id);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = BlockNode::store_count(&mut store, &to, &from);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let count = None;

    let res = BlockNode::store_list(&mut store, &to, &from, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = blocknode.store_upsert(&mut store);
    assert!(res.is_ok());

    let res = BlockNode::store_count(&mut store, &to, &from);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    let count = None;

    let res = BlockNode::store_list(&mut store, &to, &from, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![blocknode.clone()]);
}