use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::util::Version;
use mitrid_core::base::Meta;
use mitrid_core::io::Storable;

use fixture::base::eval::*;
use fixture::base::Payload;
use fixture::crypto::{Digest, Hasher};
use fixture::model::blocknode::*;
use fixture::model::blockgraph::*;
use fixture::io::store::*;

#[test]
fn test_blockgraph_meta() {
    let valid_meta = Meta::default();
    
    let res = BlockGraph::new().meta(&valid_meta);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let res = BlockGraph::new().meta(&invalid_meta);
    assert!(res.is_err())
}

#[test]
fn test_blockgraph_frontier() {
    let block_id = Digest::default();
    let block_height = 0;

    let mut bn = BlockNode::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .block_data(&block_id, block_height)
                    .unwrap();

    let tip_idx = 0;

    let res = BlockGraph::new().frontier(Some(tip_idx), &vec![bn.clone(), bn.clone()]);
    assert!(res.is_err());

    let res = BlockGraph::new().frontier(Some(tip_idx), &vec![bn.clone()]);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;
    bn.meta = invalid_meta;

    let res = BlockGraph::new().frontier(Some(tip_idx), &vec![bn]);
    assert!(res.is_err());
}

#[test]
fn test_blockgraph_payload() {
    let payload = Payload::default();

    let res = BlockGraph::new().payload(&payload);
    assert!(res.is_ok())
}

#[test]
fn test_blockgraph_digest() {
    let bg = BlockGraph::new();

    let mut hasher = Hasher{};

    let res = bg.digest(&mut hasher);
    assert!(res.is_ok());
}

#[test]
fn test_blockgraph_verify_digest() {
    let mut bg = BlockGraph::new();

    let mut hasher = Hasher{};

    bg.id = bg.digest(&mut hasher).unwrap();
    
    let res = bg.verify_digest(&mut hasher);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_blockgraph_check_digest() {
    let mut bg = BlockGraph::new();

    let mut hasher = Hasher{};

    bg.id = bg.digest(&mut hasher).unwrap();
    
    let res = bg.check_digest(&mut hasher);
    assert!(res.is_ok())
}

#[test]
fn test_blockgraph_finalize() {
    let block_height = 0;

    let bn = BlockNode::new()
                .meta(&Meta::default())
                .unwrap()
                .block_data(&Digest::default(), block_height)
                .unwrap();

    let tip_idx = 0;

    let mut bg = BlockGraph::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .frontier(Some(tip_idx), &vec![bn])
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap();

    let mut hasher = Hasher{};

    let res = bg.clone().finalize(&mut hasher);
    assert!(res.is_ok());

    bg.tip_idx = Some(bg.frontier_len + 1);

    let res = bg.finalize(&mut hasher);
    assert!(res.is_err());
}

#[test]
fn test_blockgraph_check() {
    let block_height = 0;

    let bn = BlockNode::new()
                .meta(&Meta::default())
                .unwrap()
                .block_data(&Digest::default(), block_height)
                .unwrap();

    let tip_id = 0;

    let mut hasher = Hasher{};

    let mut bg = BlockGraph::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .frontier(Some(tip_id), &vec![bn])
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let res = bg.check();
    assert!(res.is_ok());

    bg.tip_idx = Some(bg.frontier_len + 1);

    let res = bg.check();
    assert!(res.is_err());
}

#[test]
fn test_blockgraph_eval() {
    let block_height = 0;

    let bn = BlockNode::new()
                .meta(&Meta::default())
                .unwrap()
                .block_data(&Digest::default(), block_height)
                .unwrap();

    let tip_id = 0;

    let payload = Payload::new("pAyLoAd");

    let mut hasher = Hasher{};

    let mut bg = BlockGraph::new()
                .meta(&Meta::default())
                .unwrap()
                .frontier(Some(tip_id), &vec![bn])
                .unwrap()
                .payload(&payload)
                .unwrap()
                .finalize(&mut hasher)
                .unwrap();

    let mut evaluator = BlockGraphEvaluator{};

    let res = bg.eval(&PayloadEvalParams::Const, &evaluator);
    assert!(res.is_ok());

    let const_res = res.unwrap();
    assert_eq!(const_res, PayloadEvalResult::Const(payload.to_string()));

    let res = bg.eval_mut(&PayloadEvalMutParams::ToUppercase, &mut evaluator);
    assert!(res.is_ok());

    let to_uppercase_res = res.unwrap();

    let uppsercase_payload = payload.to_string().to_uppercase();
    assert_eq!(to_uppercase_res, PayloadEvalMutResult::ToUppercase(uppsercase_payload.clone()));
    assert_eq!(bg.payload.to_string(), uppsercase_payload)
}

#[test]
fn test_blockgraph_size() {
    let bg = BlockGraph::new();

    let meta_size = bg.meta.get_size();
    let bg_size = bg.size();

    assert_eq!(meta_size, bg_size);
}

#[test]
fn test_blockgraph_json() {
    let bg_a = BlockGraph::new();

    let res = bg_a.to_json();
    assert!(res.is_ok());

    let bg_json = res.unwrap();

    let res = BlockGraph::from_json(&bg_json);
    assert!(res.is_ok());

    let bg_b = res.unwrap();

    assert_eq!(bg_a, bg_b);
}

#[test]
fn test_blockgraph_bytes() {
    let bg_a = BlockGraph::new();

    let res = bg_a.to_bytes();
    assert!(res.is_ok());

    let bg_bytes = res.unwrap();

    let res = BlockGraph::from_bytes(&bg_bytes);
    assert!(res.is_ok());

    let bg_b = res.unwrap();

    assert_eq!(bg_a, bg_b);
}

#[test]
fn test_blockgraph_hex() {
    let bg_a = BlockGraph::new();

    let res = bg_a.to_hex();
    assert!(res.is_ok());

    let bg_hex = res.unwrap();

    let res = BlockGraph::from_hex(&bg_hex);
    assert!(res.is_ok());

    let bg_b = res.unwrap();

    assert_eq!(bg_a, bg_b);
}

#[test]
fn test_blockgraph_store() {
    let block_height = 0;

    let bn = BlockNode::new()
                .meta(&Meta::default())
                .unwrap()
                .block_data(&Digest::default(), block_height)
                .unwrap();

    let tip_id = 0;

    let mut hasher = Hasher{};

    let bg = BlockGraph::new()
                .meta(&Meta::default())
                .unwrap()
                .frontier(Some(tip_id), &vec![bn])
                .unwrap()
                .payload(&Payload::default())
                .unwrap()
                .finalize(&mut hasher)
                .unwrap();

    let mut store = Store::new();
    let res = bg.store_create(&mut store);
    assert!(res.is_ok());

    let res = bg.store_create(&mut store);
    assert!(res.is_err());

    let mut invalid_bg = bg.clone();
    invalid_bg.tip_idx = Some(bg.frontier_len + 1);

    let res = invalid_bg.store_create(&mut store);
    assert!(res.is_err());

    let res = BlockGraph::store_lookup(&mut store, &bg.id);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let unknown_id = Digest::default();

    let res = BlockGraph::store_lookup(&mut store, &unknown_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = BlockGraph::store_get(&mut store, &bg.id);
    assert!(res.is_ok());

    let found_bg = res.unwrap();
    assert_eq!(found_bg, bg);

    let res = BlockGraph::store_get(&mut store, &unknown_id);
    assert!(res.is_err());

    let mut from = Some(bg.id.clone());
    let mut to = Some(bg.id.clone());

    let res = BlockGraph::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_err());

    from = None;
    to = None;

    let res = BlockGraph::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(bg.id.clone());

    let res = BlockGraph::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = None;
    to = Some(bg.id.clone());

    let res = BlockGraph::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let mut from = Some(bg.id.clone());
    let mut to = Some(bg.id.clone());
    let mut count = None;
    let skip = 0;

    let res = BlockGraph::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_err());

    count = Some(0);

    let res = BlockGraph::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_err());

    from = None;
    to = None;
    count = None;

    let res = BlockGraph::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![bg.clone()]);

    from = Some(bg.id.clone());

    let res = BlockGraph::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![bg.clone()]);

    from = None;
    to = Some(bg.id.clone());

    let res = BlockGraph::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = bg.store_delete(&mut store);
    assert!(res.is_ok());

    let res = bg.store_delete(&mut store);
    assert!(res.is_err());

    let res = BlockGraph::store_lookup(&mut store, &bg.id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = BlockGraph::store_get(&mut store, &bg.id);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = BlockGraph::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let count = None;

    let res = BlockGraph::store_list(&mut store, to.clone(), from.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = bg.store_upsert(&mut store);
    assert!(res.is_ok());

    let res = BlockGraph::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    let count = None;

    let res = BlockGraph::store_list(&mut store, to, from, count, skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![bg.clone()]);
}