use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::crypto::Sign;
use mitrid_core::util::Version;
use mitrid_core::base::Meta;
use mitrid_core::io::Storable;

use fixture::base::eval::*;
use fixture::base::Payload;
use fixture::crypto::{Digest, Hasher};
use fixture::crypto::Signer;
use fixture::model::wallet::*;
use fixture::io::store::*;

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
    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let wallet = Wallet::new();

    let res = wallet.sign(&sk, &pk, &mut signer);

    assert!(res.is_ok());
}

#[test]
fn test_wallet_verify_sign() {
    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let mut wallet = Wallet::new()
                    .sign(&sk, &pk, &mut signer)
                    .unwrap();

    let res = wallet.verify_signature(&mut signer);

    assert!(res.is_ok());
    assert!(res.unwrap());

    wallet = wallet.payload(&Payload::new("a different payload")).unwrap();
    let res = wallet.verify_signature(&mut signer);
    assert!(res.is_ok());
    assert!(!res.unwrap());
}

#[test]
fn test_wallet_check_sign() {
    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let mut wallet = Wallet::new()
                        .sign(&sk, &pk, &mut signer)
                        .unwrap();

    let res = wallet.check_signature(&mut signer);
    assert!(res.is_ok());

    wallet = wallet.payload(&Payload::new("a different payload")).unwrap();
    let res = wallet.check_signature(&mut signer);
    assert!(res.is_err());
}

#[test]
fn test_wallet_digest() {
    let wallet = Wallet::new();

    let mut hasher = Hasher{};

    let res = wallet.digest(&mut hasher);
    assert!(res.is_ok());
}

#[test]
fn test_wallet_verify_digest() {
    let mut wallet = Wallet::new();

    let mut hasher = Hasher{};

    wallet.id = wallet.digest(&mut hasher).unwrap();
    
    let res = wallet.verify_digest(&mut hasher);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_wallet_check_digest() {
    let mut wallet = Wallet::new();

    let mut hasher = Hasher{};

    wallet.id = wallet.digest(&mut hasher).unwrap();
    
    let res = wallet.check_digest(&mut hasher);
    assert!(res.is_ok())
}

#[test]
fn test_wallet_finalize() {
    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let mut wallet = Wallet::new()
                        .meta(&Meta::default())
                        .unwrap()
                        .payload(&Payload::default())
                        .unwrap()
                        .sign(&sk, &pk, &mut signer)
                        .unwrap();

    let mut hasher = Hasher{};

    let res = wallet.clone().finalize(&mut hasher);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;
    wallet.meta = invalid_meta;

    let res = wallet.finalize(&mut hasher);
    assert!(res.is_err());
}

#[test]
fn test_wallet_check() {
    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let mut hasher = Hasher{};

    let mut wallet = Wallet::new()
                        .meta(&Meta::default())
                        .unwrap()
                        .payload(&Payload::default())
                        .unwrap()
                        .sign(&sk, &pk, &mut signer)
                        .unwrap()
                        .finalize(&mut hasher)
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

    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let mut hasher = Hasher{};

    let mut wallet = Wallet::new()
                        .meta(&Meta::default())
                        .unwrap()
                        .payload(&payload)
                        .unwrap()
                        .sign(&sk, &pk, &mut signer)
                        .unwrap()
                        .finalize(&mut hasher)
                        .unwrap();

    let mut evaluator = WalletEvaluator{};

    let res = wallet.eval(&PayloadEvalParams::Const, &evaluator);
    assert!(res.is_ok());

    let const_res = res.unwrap();
    assert_eq!(const_res, PayloadEvalResult::Const(payload.to_string()));

    let res = wallet.eval_mut(&PayloadEvalMutParams::ToUppercase, &mut evaluator);
    assert!(res.is_ok());

    let to_uppercase_res = res.unwrap();

    let uppsercase_payload = payload.to_string().to_uppercase();
    assert_eq!(to_uppercase_res, PayloadEvalMutResult::ToUppercase(uppsercase_payload.clone()));
    assert_eq!(wallet.payload.to_string(), uppsercase_payload)
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
    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let mut hasher = Hasher{};

    let wallet = Wallet::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .sign(&sk, &pk, &mut signer)
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let mut store = Store::new();
    let res = wallet.store_create(&mut store);
    assert!(res.is_ok());

    let res = wallet.store_create(&mut store);
    assert!(res.is_err());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let mut invalid_wallet = wallet.clone();
    invalid_wallet.meta = invalid_meta;

    let res = invalid_wallet.store_create(&mut store);
    assert!(res.is_err());

    let res = Wallet::store_lookup(&mut store, &wallet.id);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let unknown_id = Digest::default();

    let res = Wallet::store_lookup(&mut store, &unknown_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Wallet::store_get(&mut store, &wallet.id);
    assert!(res.is_ok());

    let found_wallet = res.unwrap();
    assert_eq!(found_wallet, wallet);

    let res = Wallet::store_get(&mut store, &unknown_id);
    assert!(res.is_err());

    let mut from = Some(wallet.id.clone());
    let mut to = Some(wallet.id.clone());

    let res = Wallet::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Wallet::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(wallet.id.clone());

    let res = Wallet::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = None;
    to = Some(wallet.id.clone());

    let res = Wallet::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let mut from = Some(wallet.id.clone());
    let mut to = Some(wallet.id.clone());
    let mut count = None;

    let res = Wallet::store_list(&mut store, from.clone(), to.clone(), count.clone());
    assert!(res.is_err());

    count = Some(0);

    let res = Wallet::store_list(&mut store, from.clone(), to.clone(), count.clone());
    assert!(res.is_err());

    from = None;
    to = None;
    count = None;

    let res = Wallet::store_list(&mut store, from.clone(), to.clone(), count.clone());
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![wallet.clone()]);

    from = Some(wallet.id.clone());

    let res = Wallet::store_list(&mut store, from.clone(), to.clone(), count.clone());
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![wallet.clone()]);

    from = None;
    to = Some(wallet.id.clone());

    let res = Wallet::store_list(&mut store, from.clone(), to.clone(), count.clone());
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = wallet.store_delete(&mut store);
    assert!(res.is_ok());

    let res = wallet.store_delete(&mut store);
    assert!(res.is_err());

    let res = Wallet::store_lookup(&mut store, &wallet.id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Wallet::store_get(&mut store, &wallet.id);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Wallet::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let count = None;

    let res = Wallet::store_list(&mut store, to.clone(), from.clone(), count.clone());
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = wallet.store_upsert(&mut store);
    assert!(res.is_ok());

    let res = Wallet::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    let count = None;

    let res = Wallet::store_list(&mut store, to, from, count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![wallet.clone()]);
}