use mitrid_core::utils::Version;
use mitrid_core::models::Meta;
/*
use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
*/
use mitrid_core::base::Serializable;

use fixtures::crypto::Digest;
use fixtures::models::Amount;

use fixtures::crypto::SHA512;
use fixtures::models::Coin;

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
    let valid_tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let res = Coin::new().output_data(&valid_tx_id, out_idx, &out_amount);
    assert!(res.is_ok())
}

#[test]
fn test_coin_finalize() {
    let valid_meta = Meta::default();
    let valid_tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let coin = Coin::new()
                .meta(&valid_meta)
                .unwrap()
                .output_data(&valid_tx_id, out_idx, &out_amount)
                .unwrap();

    let coin_bytes = coin.to_bytes().unwrap();
    let res = coin.finalize(&coin_bytes,
                            &|_, msg| {
                                SHA512::digest(&msg)
                            });

    println!("res: {:?}", res.clone());
    assert!(res.is_ok());
}

#[test]
fn test_verify_digest() {}

#[test]
fn test_check_digest() {}

#[test]
fn test_coin_size() {}

#[test]
fn test_coin_check() {}

#[test]
fn test_coin_json() {}

#[test]
fn test_coin_bytes() {}

#[test]
fn test_coin_hex() {}