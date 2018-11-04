use mitrid_core::utils::Version;
use mitrid_core::base::Meta;
use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::io::Storable;

use fixtures::crypto::Digest;
use fixtures::crypto::SHA512HMAC;
use fixtures::models::Amount;
use fixtures::models::coin::*;
use fixtures::io::store::*;

#[test]
fn test_coin_meta() {
    let valid_meta = Meta::default();
    
    let res = Coin::new().meta(&valid_meta);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let res = Coin::new().meta(&invalid_meta);
    assert!(res.is_err())
}

#[test]
fn test_coin_output() {
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let res = Coin::new().output_data(&tx_id, out_idx, &out_amount);
    assert!(res.is_ok())
}

#[test]
fn test_digest() {
    let coin = Coin::new();

    let res = coin.digest(&(), &coin_digest_cb);
    assert!(res.is_ok());
}

#[test]
fn test_verify_digest() {
    let mut coin = Coin::new();

    coin.id = coin.digest(&(), &coin_digest_cb).unwrap();
    
    let res = coin.verify_digest(&(), &coin_verify_digest_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_check_digest() {
    let mut coin = Coin::new();

    coin.id = coin.digest(&(), &coin_digest_cb).unwrap();
    
    let res = coin.check_digest(&(), &coin_check_digest_cb);
    assert!(res.is_ok())
}

#[test]
fn test_coin_commit() {
    let coin = Coin::new();

    let res = coin.commit(&(), &coin_commit_cb);
    assert!(res.is_ok());
}

#[test]
fn test_coin_verify_commitment() {
    let coin = Coin::new();

    let commitment = coin.commit(&(), &coin_commit_cb).unwrap();
    
    let res = coin.verify_commitment(&(), &commitment, &coin_verify_commitment_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_coin_check_commitment() {
    let coin = Coin::new();

    let commitment = coin.commit(&(), &coin_commit_cb).unwrap();
    
    let res = coin.check_commitment(&(), &commitment, &coin_check_commitment_cb);
    assert!(res.is_ok())
}

#[test]
fn test_coin_authenticate() {
    let coin = Coin::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let res = coin.authenticate(&key, &coin_authenticate_cb);
    assert!(res.is_ok());
}

#[test]
fn test_coin_verify_tag() {
    let coin = Coin::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let res = coin.authenticate(&key, &coin_authenticate_cb);
    assert!(res.is_ok());

    let tag = res.unwrap();
    
    let res = coin.verify_tag(&key, &tag, &coin_verify_tag_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_coin_check_tag() {
    let coin = Coin::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let res = coin.authenticate(&key, &coin_authenticate_cb);
    assert!(res.is_ok());

    let tag = res.unwrap();
    
    let res = coin.check_tag(&key, &tag, &coin_check_tag_cb);
    assert!(res.is_ok())
}

#[test]
fn test_coin_finalize() {
    let valid_meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let mut coin = Coin::new()
                    .meta(&valid_meta)
                    .unwrap()
                    .output_data(&tx_id, out_idx, &out_amount)
                    .unwrap();

    let res = coin.clone().finalize(&(), &coin_digest_cb);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    coin.meta = invalid_meta;

    let res = coin.finalize(&(), &coin_digest_cb);
    assert!(res.is_err());
}

#[test]
fn test_coin_check() {
    let valid_meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let mut coin = Coin::new()
                    .meta(&valid_meta)
                    .unwrap()
                    .output_data(&tx_id, out_idx, &out_amount)
                    .unwrap()
                    .finalize(&(), &coin_digest_cb)
                    .unwrap();

    let res = coin.clone().check();
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    coin.meta = invalid_meta;

    let res = coin.check();
    assert!(res.is_err());
}

#[test]
fn test_coin_size() {
    let coin = Coin::new();

    let coin_size = coin.size();

    let meta_size = coin.meta.get_size();

    assert_eq!(meta_size, coin_size);
}

#[test]
fn test_coin_json() {
    let coin_a = Coin::new();

    let res = coin_a.to_json();
    assert!(res.is_ok());

    let coin_json = res.unwrap();

    let res = Coin::from_json(&coin_json);
    assert!(res.is_ok());

    let coin_b = res.unwrap();

    assert_eq!(coin_a, coin_b)
}

#[test]
fn test_coin_bytes() {
    let coin_a = Coin::new();

    let res = coin_a.to_bytes();
    assert!(res.is_ok());

    let coin_bytes = res.unwrap();

    let res = Coin::from_bytes(&coin_bytes);
    assert!(res.is_ok());

    let coin_b = res.unwrap();

    assert_eq!(coin_a, coin_b)
}

#[test]
fn test_coin_hex() {
    let coin_a = Coin::new();

    let res = coin_a.to_hex();
    assert!(res.is_ok());

    let coin_hex = res.unwrap();

    let res = Coin::from_hex(&coin_hex);
    assert!(res.is_ok());

    let coin_b = res.unwrap();

    assert_eq!(coin_a, coin_b)
}

#[test]
fn test_coin_store() {
    let valid_meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let coin = Coin::new()
                    .meta(&valid_meta)
                    .unwrap()
                    .output_data(&tx_id, out_idx, &out_amount)
                    .unwrap()
                    .finalize(&(), &coin_digest_cb)
                    .unwrap();

    let mut store = Store::new();
    let res = coin.store_create(&mut store, &());
    assert!(res.is_ok());

    let res = coin.store_create(&mut store, &());
    assert!(res.is_err());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let mut invalid_coin = coin.clone();

    invalid_coin.meta = invalid_meta;

    let res = invalid_coin.store_create(&mut store, &());
    assert!(res.is_err());

    let res = Coin::store_lookup(&mut store, &(), &coin.id);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let unknown_id = Digest::default();

    let res = Coin::store_lookup(&mut store, &(), &unknown_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Coin::store_get(&mut store, &(), &coin.id);
    assert!(res.is_ok());

    let found_coin = res.unwrap();
    assert_eq!(found_coin, coin);

    let res = Coin::store_get(&mut store, &(), &unknown_id);
    assert!(res.is_err());

    let mut from = Some(coin.id.clone());
    let mut to = Some(coin.id.clone());

    let res = Coin::store_count(&mut store, &(), &from, &to);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Coin::store_count(&mut store, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(coin.id.clone());

    let res = Coin::store_count(&mut store, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = None;
    to = Some(coin.id.clone());

    let res = Coin::store_count(&mut store, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let mut from = Some(coin.id.clone());
    let mut to = Some(coin.id.clone());
    let mut count = None;

    let res = Coin::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_err());

    count = Some(0);

    let res = Coin::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_err());

    from = None;
    to = None;
    count = None;

    let res = Coin::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![coin.clone()]);

    from = Some(coin.id.clone());

    let res = Coin::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![coin.clone()]);

    from = None;
    to = Some(coin.id.clone());

    let res = Coin::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = coin.store_delete(&mut store, &());
    assert!(res.is_ok());

    let res = coin.store_delete(&mut store, &());
    assert!(res.is_err());

    let res = Coin::store_lookup(&mut store, &(), &coin.id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Coin::store_get(&mut store, &(), &coin.id);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Coin::store_count(&mut store, &(), &to, &from);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let count = None;

    let res = Coin::store_list(&mut store, &(), &to, &from, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = coin.store_upsert(&mut store, &());
    assert!(res.is_ok());

    let res = Coin::store_count(&mut store, &(), &to, &from);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    let count = None;

    let res = Coin::store_list(&mut store, &(), &to, &from, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![coin.clone()]);
}