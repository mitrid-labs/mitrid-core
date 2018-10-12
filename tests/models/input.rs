use mitrid_core::utils::Version;
use mitrid_core::models::Meta;
use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;

use fixtures::base::eval::*;
use fixtures::crypto::Digest;
use fixtures::crypto::{SecretKey, Ed25519};
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
fn test_input_sign() {
    let (pk, sk) = Ed25519::keypair(&None).unwrap();

    let input = Input::new();

    let res = input.sign(&(), &sk, &pk, &input_sign_cb);

    assert!(res.is_ok());
}

#[test]
fn test_input_verify_sign() {
    let (pk, sk) = Ed25519::keypair(&None).unwrap();

    let mut input = Input::new()
                    .sign(&(), &sk, &pk, &input_sign_cb)
                    .unwrap();

    let res = input.verify_signature::<(), SecretKey>(&(), &input_verify_signature_cb);

    assert!(res.is_ok());
    assert!(res.unwrap());

    input = input.payload(&Payload::new("a different payload")).unwrap();
    let res = input.verify_signature::<(), SecretKey>(&(), &input_verify_signature_cb);
    assert!(res.is_ok());
    assert!(!res.unwrap());
}

#[test]
fn test_input_check_sign() {
    let (pk, sk) = Ed25519::keypair(&None).unwrap();

    let mut input = Input::new()
                        .sign(&(), &sk, &pk, &input_sign_cb)
                        .unwrap();

    let res = input.check_signature::<(), SecretKey>(&(), &input_check_signature_cb);
    assert!(res.is_ok());

    input = input.payload(&Payload::new("a different payload")).unwrap();
    let res = input.check_signature::<(), SecretKey>(&(), &input_check_signature_cb);
    assert!(res.is_err());
}

#[test]
fn test_input_digest() {
    let input = Input::new();

    let res = input.digest(&(), &input_digest_cb);
    assert!(res.is_ok());
}

#[test]
fn test_input_verify_digest() {
    let mut input = Input::new();

    input.id = input.digest(&(), &input_digest_cb).unwrap();
    
    let res = input.verify_digest(&(), &input_verify_digest_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_input_check_digest() {
    let mut input = Input::new();

    input.id = input.digest(&(), &input_digest_cb).unwrap();
    
    let res = input.check_digest(&(), &input_check_digest_cb);
    assert!(res.is_ok())
}

#[test]
fn test_input_finalize() {
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

    let payload = Payload::default();

    let (pk, sk) = Ed25519::keypair(&None).unwrap();

    let mut input = Input::new()
                        .meta(&meta)
                        .unwrap()
                        .coin(&coin)
                        .unwrap()
                        .payload(&payload)
                        .unwrap()
                        .sign(&(), &sk, &pk, &input_sign_cb)
                        .unwrap();

    let res = input.clone().finalize(&(), &input_digest_cb);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    input.meta = invalid_meta;

    let res = input.finalize(&(), &input_digest_cb);
    assert!(res.is_err());
}

#[test]
fn test_input_check() {
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

    let payload = Payload::default();

    let (pk, sk) = Ed25519::keypair(&None).unwrap();

    let mut input = Input::new()
                        .meta(&meta)
                        .unwrap()
                        .coin(&coin)
                        .unwrap()
                        .payload(&payload)
                        .unwrap()
                        .sign(&(), &sk, &pk, &input_sign_cb)
                        .unwrap()
                        .finalize(&(), &input_digest_cb)
                        .unwrap();

    let res = input.check();
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    input.meta = invalid_meta;

    let res = input.check();
    assert!(res.is_err());
}

#[test]
fn test_input_eval() {
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

    let payload = Payload::new("pAyLoAd");

    let (pk, sk) = Ed25519::keypair(&None).unwrap();

    let input = Input::new()
                    .meta(&meta)
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&payload)
                    .unwrap()
                    .sign(&(), &sk, &pk, &input_sign_cb)
                    .unwrap()
                    .finalize(&(), &input_digest_cb)
                    .unwrap();

    let res = input.eval(&EvalParams::Const, &input_eval_cb);
    assert!(res.is_ok());

    let const_res = res.unwrap();
    assert_eq!(const_res, EvalReturn::Const(payload.to_string()));

    let res = input.eval(&EvalParams::ToUppercase, &input_eval_cb);
    assert!(res.is_ok());

    let to_uppercase_res = res.unwrap();
    assert_eq!(to_uppercase_res, EvalReturn::ToUppercase(payload.to_string().to_uppercase()));
}

#[test]
fn test_input_size() {
    let input = Input::new();

    let meta_size = input.meta.get_size();
    let input_size = input.size();

    assert_eq!(meta_size, input_size);
}

#[test]
fn test_input_json() {
    let input_a = Input::new();

    let res = input_a.to_json();
    assert!(res.is_ok());

    let input_json = res.unwrap();

    let res = Input::from_json(&input_json);
    assert!(res.is_ok());

    let input_b = res.unwrap();

    assert_eq!(input_a, input_b);
}

#[test]
fn test_input_bytes() {
    let input_a = Input::new();

    let res = input_a.to_bytes();
    assert!(res.is_ok());

    let input_bytes = res.unwrap();

    let res = Input::from_bytes(&input_bytes);
    assert!(res.is_ok());

    let input_b = res.unwrap();

    assert_eq!(input_a, input_b);
}

#[test]
fn test_input_hex() {
    let input_a = Input::new();

    let res = input_a.to_hex();
    assert!(res.is_ok());

    let input_hex = res.unwrap();

    let res = Input::from_hex(&input_hex);
    assert!(res.is_ok());

    let input_b = res.unwrap();

    assert_eq!(input_a, input_b);
}