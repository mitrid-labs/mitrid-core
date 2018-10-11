use mitrid_core::utils::Version;
use mitrid_core::models::Meta;
use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;

use fixtures::crypto::Digest;
use fixtures::models::Amount;
use fixtures::models::coin::*;

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
    let meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let coin = Coin::new()
                .meta(&meta)
                .unwrap()
                .output_data(&tx_id, out_idx, &out_amount)
                .unwrap();

    let res = coin.digest(&(), &coin_digest_cb);
    assert!(res.is_ok());
}

#[test]
fn test_verify_digest() {
    let meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let coin = Coin::new()
                .meta(&meta)
                .unwrap()
                .output_data(&tx_id, out_idx, &out_amount)
                .unwrap();

    let digest = coin.digest(&(), &coin_digest_cb).unwrap();
    
    let res = coin.verify_digest(&(), &digest, &coin_verify_digest_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_check_digest() {
    let meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let coin = Coin::new()
                .meta(&meta)
                .unwrap()
                .output_data(&tx_id, out_idx, &out_amount)
                .unwrap();

    let digest = coin.digest(&(), &coin_digest_cb).unwrap();
    
    let res = coin.check_digest(&(), &digest, &coin_check_digest_cb);
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
fn test_coin_size() {
    let meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let coin = Coin::new()
                .meta(&meta)
                .unwrap()
                .output_data(&tx_id, out_idx, &out_amount)
                .unwrap()
                .finalize(&(), &coin_digest_cb)
                .unwrap();

    let coin_size = coin.id.size() +
                        coin.meta.size() +
                        coin.tx_id.size() +
                        coin.out_idx.size() +
                        coin.out_amount.size();

    let meta_size = coin.meta.get_size();

    assert_eq!(coin_size, coin.size());
    assert_eq!(meta_size, coin.size());
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
fn test_coin_json() {
    let meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let coin_a = Coin::new()
                    .meta(&meta)
                    .unwrap()
                    .output_data(&tx_id, out_idx, &out_amount)
                    .unwrap()
                    .finalize(&(), &coin_digest_cb)
                    .unwrap();

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
    let meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let coin_a = Coin::new()
                    .meta(&meta)
                    .unwrap()
                    .output_data(&tx_id, out_idx, &out_amount)
                    .unwrap()
                    .finalize(&(), &coin_digest_cb)
                    .unwrap();

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
    let meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let coin_a = Coin::new()
                    .meta(&meta)
                    .unwrap()
                    .output_data(&tx_id, out_idx, &out_amount)
                    .unwrap()
                    .finalize(&(), &coin_digest_cb)
                    .unwrap();

    let res = coin_a.to_hex();
    assert!(res.is_ok());

    let coin_hex = res.unwrap();

    let res = Coin::from_hex(&coin_hex);
    assert!(res.is_ok());

    let coin_b = res.unwrap();

    assert_eq!(coin_a, coin_b)
}