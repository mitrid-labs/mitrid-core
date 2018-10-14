use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::utils::Version;
use mitrid_core::models::Meta;

use fixtures::base::eval::*;
use fixtures::crypto::Ed25519;
use fixtures::models::Payload;
use fixtures::models::wallet::*;

#[test]
fn test_wallet_meta() {
    let valid_meta = Meta::default();
    
    let res = Wallet::new().meta(&valid_meta);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let res = Wallet::new().meta(&invalid_meta);
    assert!(res.is_err())
}

#[test]
fn test_wallet_payload() {
    let payload = Payload::default();

    let res = Wallet::new().payload(&payload);
    assert!(res.is_ok())
}

#[test]
fn test_wallet_sign() {
    let (pk, sk) = Ed25519::keypair(None).unwrap();

    let wallet = Wallet::new();

    let res = wallet.sign(&(), &sk, &pk, &wallet_sign_cb);

    assert!(res.is_ok());
}

#[test]
fn test_wallet_verify_sign() {
    let (pk, sk) = Ed25519::keypair(None).unwrap();

    let mut wallet = Wallet::new()
                    .sign(&(), &sk, &pk, &wallet_sign_cb)
                    .unwrap();

    let res = wallet.verify_signature(&(), &wallet_verify_signature_cb);

    assert!(res.is_ok());
    assert!(res.unwrap());

    wallet = wallet.payload(&Payload::new("a different payload")).unwrap();
    let res = wallet.verify_signature(&(), &wallet_verify_signature_cb);
    assert!(res.is_ok());
    assert!(!res.unwrap());
}

#[test]
fn test_wallet_check_sign() {
    let (pk, sk) = Ed25519::keypair(None).unwrap();

    let mut wallet = Wallet::new()
                        .sign(&(), &sk, &pk, &wallet_sign_cb)
                        .unwrap();

    let res = wallet.check_signature(&(), &wallet_check_signature_cb);
    assert!(res.is_ok());

    wallet = wallet.payload(&Payload::new("a different payload")).unwrap();
    let res = wallet.check_signature(&(), &wallet_check_signature_cb);
    assert!(res.is_err());
}

#[test]
fn test_wallet_digest() {
    let wallet = Wallet::new();

    let res = wallet.digest(&(), &wallet_digest_cb);
    assert!(res.is_ok());
}

#[test]
fn test_wallet_verify_digest() {
    let mut wallet = Wallet::new();

    wallet.id = wallet.digest(&(), &wallet_digest_cb).unwrap();
    
    let res = wallet.verify_digest(&(), &wallet_verify_digest_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_wallet_check_digest() {
    let mut wallet = Wallet::new();

    wallet.id = wallet.digest(&(), &wallet_digest_cb).unwrap();
    
    let res = wallet.check_digest(&(), &wallet_check_digest_cb);
    assert!(res.is_ok())
}

#[test]
fn test_wallet_finalize() {
    let (pk, sk) = Ed25519::keypair(None).unwrap();

    let mut wallet = Wallet::new()
                        .meta(&Meta::default())
                        .unwrap()
                        .payload(&Payload::default())
                        .unwrap()
                        .sign(&(), &sk, &pk, &wallet_sign_cb)
                        .unwrap();

    let res = wallet.clone().finalize(&(), &wallet_digest_cb);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;
    wallet.meta = invalid_meta;

    let res = wallet.finalize(&(), &wallet_digest_cb);
    assert!(res.is_err());
}

#[test]
fn test_wallet_check() {
    let (pk, sk) = Ed25519::keypair(None).unwrap();

    let mut wallet = Wallet::new()
                        .meta(&Meta::default())
                        .unwrap()
                        .payload(&Payload::default())
                        .unwrap()
                        .sign(&(), &sk, &pk, &wallet_sign_cb)
                        .unwrap()
                        .finalize(&(), &wallet_digest_cb)
                        .unwrap();

    let res = wallet.check();
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;
    wallet.meta = invalid_meta;

    let res = wallet.check();
    assert!(res.is_err());
}

#[test]
fn test_wallet_eval() {
    let payload = Payload::new("pAyLoAd");

    let (pk, sk) = Ed25519::keypair(None).unwrap();

    let wallet = Wallet::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .payload(&payload)
                    .unwrap()
                    .sign(&(), &sk, &pk, &wallet_sign_cb)
                    .unwrap()
                    .finalize(&(), &wallet_digest_cb)
                    .unwrap();

    let res = wallet.eval(&EvalParams::Const, &wallet_eval_cb);
    assert!(res.is_ok());

    let const_res = res.unwrap();
    assert_eq!(const_res, EvalReturn::Const(payload.to_string()));

    let res = wallet.eval(&EvalParams::ToUppercase, &wallet_eval_cb);
    assert!(res.is_ok());

    let to_uppercase_res = res.unwrap();
    assert_eq!(to_uppercase_res, EvalReturn::ToUppercase(payload.to_string().to_uppercase()));
}

#[test]
fn test_wallet_size() {
    let wallet = Wallet::new();

    let meta_size = wallet.meta.get_size();
    let wallet_size = wallet.size();

    assert_eq!(meta_size, wallet_size);
}

#[test]
fn test_wallet_json() {
    let wallet_a = Wallet::new();

    let res = wallet_a.to_json();
    assert!(res.is_ok());

    let wallet_json = res.unwrap();

    let res = Wallet::from_json(&wallet_json);
    assert!(res.is_ok());

    let wallet_b = res.unwrap();

    assert_eq!(wallet_a, wallet_b);
}

#[test]
fn test_wallet_bytes() {
    let wallet_a = Wallet::new();

    let res = wallet_a.to_bytes();
    assert!(res.is_ok());

    let wallet_bytes = res.unwrap();

    let res = Wallet::from_bytes(&wallet_bytes);
    assert!(res.is_ok());

    let wallet_b = res.unwrap();

    assert_eq!(wallet_a, wallet_b);
}

#[test]
fn test_wallet_hex() {
    let wallet_a = Wallet::new();

    let res = wallet_a.to_hex();
    assert!(res.is_ok());

    let wallet_hex = res.unwrap();

    let res = Wallet::from_hex(&wallet_hex);
    assert!(res.is_ok());

    let wallet_b = res.unwrap();

    assert_eq!(wallet_a, wallet_b);
}