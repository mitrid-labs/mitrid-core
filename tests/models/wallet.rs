use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::utils::Version;
use mitrid_core::models::Meta;
use mitrid_core::io::Storable;

use fixtures::base::eval::*;
use fixtures::base::Payload;
use fixtures::crypto::Digest;
use fixtures::crypto::Ed25519;
use fixtures::crypto::SHA512HMAC;
use fixtures::models::wallet::*;
use fixtures::io::store::*;

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
fn test_wallet_commit() {
    let wallet = Wallet::new();

    let res = wallet.commit(&(), &wallet_commit_cb);
    assert!(res.is_ok());
}

#[test]
fn test_wallet_verify_commitment() {
    let wallet = Wallet::new();

    let commitment = wallet.commit(&(), &wallet_commit_cb).unwrap();
    
    let res = wallet.verify_commitment(&(), &commitment, &wallet_verify_commitment_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_wallet_check_commitment() {
    let wallet = Wallet::new();

    let commitment = wallet.commit(&(), &wallet_commit_cb).unwrap();
    
    let res = wallet.check_commitment(&(), &commitment, &wallet_check_commitment_cb);
    assert!(res.is_ok())
}

#[test]
fn test_wallet_authenticate() {
    let wallet = Wallet::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let res = wallet.authenticate(&key, &wallet_authenticate_cb);
    assert!(res.is_ok());
}

#[test]
fn test_wallet_verify_tag() {
    let wallet = Wallet::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let tag = wallet.authenticate(&key, &wallet_authenticate_cb).unwrap();
    
    let res = wallet.verify_tag(&key, &tag, &wallet_verify_tag_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_wallet_check_tag() {
    let wallet = Wallet::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let tag = wallet.authenticate(&key, &wallet_authenticate_cb).unwrap();
    
    let res = wallet.check_tag(&key, &tag, &wallet_check_tag_cb);
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

#[test]
fn test_wallet_store() {
    let (pk, sk) = Ed25519::keypair(None).unwrap();

    let wallet = Wallet::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .sign(&(), &sk, &pk, &wallet_sign_cb)
                    .unwrap()
                    .finalize(&(), &wallet_digest_cb)
                    .unwrap();

    let mut store = Store::new();
    let res = wallet.store_create(&mut store, &());
    assert!(res.is_ok());

    let res = wallet.store_create(&mut store, &());
    assert!(res.is_err());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let mut invalid_wallet = wallet.clone();
    invalid_wallet.meta = invalid_meta;

    let res = invalid_wallet.store_create(&mut store, &());
    assert!(res.is_err());

    let res = Wallet::store_lookup(&mut store, &(), &wallet.id);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let unknown_id = Digest::default();

    let res = Wallet::store_lookup(&mut store, &(), &unknown_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Wallet::store_get(&mut store, &(), &wallet.id);
    assert!(res.is_ok());

    let found_wallet = res.unwrap();
    assert_eq!(found_wallet, wallet);

    let res = Wallet::store_get(&mut store, &(), &unknown_id);
    assert!(res.is_err());

    let mut from = Some(wallet.id.clone());
    let mut to = Some(wallet.id.clone());

    let res = Wallet::store_count(&mut store, &(), &from, &to);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Wallet::store_count(&mut store, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(wallet.id.clone());

    let res = Wallet::store_count(&mut store, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = None;
    to = Some(wallet.id.clone());

    let res = Wallet::store_count(&mut store, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let mut from = Some(wallet.id.clone());
    let mut to = Some(wallet.id.clone());
    let mut count = None;

    let res = Wallet::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_err());

    count = Some(0);

    let res = Wallet::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_err());

    from = None;
    to = None;
    count = None;

    let res = Wallet::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![wallet.clone()]);

    from = Some(wallet.id.clone());

    let res = Wallet::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![wallet.clone()]);

    from = None;
    to = Some(wallet.id.clone());

    let res = Wallet::store_list(&mut store, &(), &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = wallet.store_delete(&mut store, &());
    assert!(res.is_ok());

    let res = wallet.store_delete(&mut store, &());
    assert!(res.is_err());

    let res = Wallet::store_lookup(&mut store, &(), &wallet.id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Wallet::store_get(&mut store, &(), &wallet.id);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Wallet::store_count(&mut store, &(), &to, &from);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let count = None;

    let res = Wallet::store_list(&mut store, &(), &to, &from, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = wallet.store_upsert(&mut store, &());
    assert!(res.is_ok());

    let res = Wallet::store_count(&mut store, &(), &to, &from);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    let count = None;

    let res = Wallet::store_list(&mut store, &(), &to, &from, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![wallet.clone()]);
}