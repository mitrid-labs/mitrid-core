use mitrid_core::utils::Version;
use mitrid_core::models::Meta;
//use mitrid_core::base::Checkable;
//use mitrid_core::base::Sizable;
//use mitrid_core::base::Serializable;

use fixtures::crypto::Digest;
use fixtures::models::Amount;
use fixtures::models::Payload;
use fixtures::models::coin::*;
use fixtures::models::input::*;

#[test]
fn test_input_meta() {
    let valid_meta = Meta::default();
    
    let res = Input::new().meta(&valid_meta);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let res = Input::new().meta(&invalid_meta);
    assert!(res.is_err())
}

#[test]
fn test_input_coin() {
    let valid_meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let valid_coin = Coin::new()
                        .meta(&valid_meta)
                        .unwrap()
                        .output_data(&tx_id, out_idx, &out_amount)
                        .unwrap()
                        .finalize(&(), &coin_digest_cb)
                        .unwrap();

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let mut invalid_coin = valid_coin.clone();
    invalid_coin.meta = invalid_meta;

    let res = Input::new().coin(&valid_coin);
    assert!(res.is_ok());

    let res = Input::new().coin(&invalid_coin);
    assert!(res.is_err());
}

#[test]
fn test_input_payload() {
    let payload = Payload::default();

    let res = Input::new().payload(&payload);
    assert!(res.is_ok())
}

#[test]
fn test_input_sign() {}

#[test]
fn test_input_verify_sign() {}

#[test]
fn test_input_check_sign() {}

#[test]
fn test_input_finalize() {}

#[test]
fn test_input_digest() {}

#[test]
fn test_input_verify_digest() {}

#[test]
fn test_input_check_digest() {}

#[test]
fn test_input_check() {}

#[test]
fn test_input_size() {}

#[test]
fn test_input_eval() {}

#[test]
fn test_input_json() {}

#[test]
fn test_input_bytes() {}

#[test]
fn test_input_hex() {}