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
use fixture::model::Amount;
use fixture::model::coin::*;
use fixture::model::input::*;
use fixture::io::store::*;

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

    let mut hasher = Hasher{};

    let valid_coin = Coin::new()
                        .meta(&valid_meta)
                        .unwrap()
                        .output_data(&tx_id, out_idx, &out_amount)
                        .unwrap()
                        .finalize(&mut hasher)
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
    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let input = Input::new();

    let res = input.sign(&sk, &pk, &mut signer);

    assert!(res.is_ok());
}

#[test]
fn test_input_verify_sign() {
    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let mut input = Input::new()
                    .sign(&sk, &pk, &mut signer)
                    .unwrap();

    let res = input.verify_signature(&mut signer);

    assert!(res.is_ok());
    assert!(res.unwrap());

    input = input.payload(&Payload::new("a different payload")).unwrap();
    let res = input.verify_signature(&mut signer);
    assert!(res.is_ok());
    assert!(!res.unwrap());
}

#[test]
fn test_input_check_sign() {
    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let mut input = Input::new()
                        .sign(&sk, &pk, &mut signer)
                        .unwrap();

    let res = input.check_signature(&mut signer);
    assert!(res.is_ok());

    input = input.payload(&Payload::new("a different payload")).unwrap();
    let res = input.check_signature(&mut signer);
    assert!(res.is_err());
}

#[test]
fn test_input_digest() {
    let input = Input::new();

    let mut hasher = Hasher{};

    let res = input.digest(&mut hasher);
    assert!(res.is_ok());
}

#[test]
fn test_input_verify_digest() {
    let mut input = Input::new();

    let mut hasher = Hasher{};

    input.id = input.digest(&mut hasher).unwrap();
    
    let res = input.verify_digest(&mut hasher);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_input_check_digest() {
    let mut input = Input::new();

    let mut hasher = Hasher{};

    input.id = input.digest(&mut hasher).unwrap();
    
    let res = input.check_digest(&mut hasher);
    assert!(res.is_ok())
}

#[test]
fn test_input_finalize() {
    let meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .meta(&meta)
                    .unwrap()
                    .output_data(&tx_id, out_idx, &out_amount)
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let payload = Payload::default();

    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let mut input = Input::new()
                        .meta(&meta)
                        .unwrap()
                        .coin(&coin)
                        .unwrap()
                        .payload(&payload)
                        .unwrap()
                        .sign(&sk, &pk, &mut signer)
                        .unwrap();

    let res = input.clone().finalize(&mut hasher);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    input.meta = invalid_meta;

    let res = input.finalize(&mut hasher);
    assert!(res.is_err());
}

#[test]
fn test_input_check() {
    let meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .meta(&meta)
                    .unwrap()
                    .output_data(&tx_id, out_idx, &out_amount)
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let payload = Payload::default();

    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let mut input = Input::new()
                        .meta(&meta)
                        .unwrap()
                        .coin(&coin)
                        .unwrap()
                        .payload(&payload)
                        .unwrap()
                        .sign(&sk, &pk, &mut signer)
                        .unwrap()
                        .finalize(&mut hasher)
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

    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .meta(&meta)
                    .unwrap()
                    .output_data(&tx_id, out_idx, &out_amount)
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let payload = Payload::new("pAyLoAd");

    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let mut input = Input::new()
                        .meta(&meta)
                        .unwrap()
                        .coin(&coin)
                        .unwrap()
                        .payload(&payload)
                        .unwrap()
                        .sign(&sk, &pk, &mut signer)
                        .unwrap()
                        .finalize(&mut hasher)
                        .unwrap();

    let mut evaluator = InputEvaluator{};

    let res = input.eval(&PayloadEvalParams::Const, &evaluator);
    assert!(res.is_ok());

    let const_res = res.unwrap();
    assert_eq!(const_res, PayloadEvalResult::Const(payload.to_string()));

    let res = input.eval_mut(&PayloadEvalMutParams::ToUppercase, &mut evaluator);
    assert!(res.is_ok());

    let to_uppercase_res = res.unwrap();

    let uppsercase_payload = payload.to_string().to_uppercase();
    assert_eq!(to_uppercase_res, PayloadEvalMutResult::ToUppercase(uppsercase_payload.clone()));
    assert_eq!(input.payload.to_string(), uppsercase_payload)
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

#[test]
fn test_input_store() {
    let meta = Meta::default();
    let tx_id = Digest::default();
    let out_idx = 0;
    let out_amount = Amount::default();

    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .meta(&meta)
                    .unwrap()
                    .output_data(&tx_id, out_idx, &out_amount)
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let payload = Payload::default();

    let mut signer = Signer{};
    let (pk, sk) = signer.generate_keys(None).unwrap();

    let input = Input::new()
                    .meta(&meta)
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&payload)
                    .unwrap()
                    .sign(&sk, &pk, &mut signer)
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let mut store = Store::new();
    let res = input.store_create(&mut store);
    assert!(res.is_ok());

    let res = input.store_create(&mut store);
    assert!(res.is_err());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let mut invalid_input = input.clone();
    invalid_input.meta = invalid_meta;

    let res = invalid_input.store_create(&mut store);
    assert!(res.is_err());

    let res = Input::store_lookup(&mut store, &input.id);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let unknown_id = Digest::default();

    let res = Input::store_lookup(&mut store, &unknown_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Input::store_get(&mut store, &input.id);
    assert!(res.is_ok());

    let found_input = res.unwrap();
    assert_eq!(found_input, input);

    let res = Input::store_get(&mut store, &unknown_id);
    assert!(res.is_err());

    let mut from = Some(input.id.clone());
    let mut to = Some(input.id.clone());

    let res = Input::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Input::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(input.id.clone());

    let res = Input::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = None;
    to = Some(input.id.clone());

    let res = Input::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let mut from = Some(input.id.clone());
    let mut to = Some(input.id.clone());
    let mut count = None;

    let res = Input::store_list(&mut store, from.clone(), to.clone(), count.clone());
    assert!(res.is_err());

    count = Some(0);

    let res = Input::store_list(&mut store, from.clone(), to.clone(), count.clone());
    assert!(res.is_err());

    from = None;
    to = None;
    count = None;

    let res = Input::store_list(&mut store, from.clone(), to.clone(), count.clone());
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![input.clone()]);

    from = Some(input.id.clone());

    let res = Input::store_list(&mut store, from.clone(), to.clone(), count.clone());
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![input.clone()]);

    from = None;
    to = Some(input.id.clone());

    let res = Input::store_list(&mut store, from.clone(), to.clone(), count.clone());
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = input.store_delete(&mut store);
    assert!(res.is_ok());

    let res = input.store_delete(&mut store);
    assert!(res.is_err());

    let res = Input::store_lookup(&mut store, &coin.id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Input::store_get(&mut store, &coin.id);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Input::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let count = None;

    let res = Input::store_list(&mut store, to.clone(), from.clone(), count.clone());
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = input.store_upsert(&mut store);
    assert!(res.is_ok());

    let res = Input::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    let count = None;

    let res = Input::store_list(&mut store, to, from, count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![input.clone()]);
}