use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::util::Version;
use mitrid_core::base::Meta;
use mitrid_core::io::Storable;

use fixture::base::eval::*;
use fixture::base::Payload;
use fixture::crypto::{Digest, Hasher};
use fixture::model::Amount;
use fixture::model::output::*;
use fixture::io::store::*;

#[test]
fn test_output_meta() {
    let valid_meta = Meta::default();
    
    let res = Output::new().meta(&valid_meta);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let res = Output::new().meta(&invalid_meta);
    assert!(res.is_err())
}

#[test]
fn test_output_amount() {
    let amount = Amount::default();
    
    let res = Output::new().amount(&amount);
    assert!(res.is_ok());
}

#[test]
fn test_output_payload() {
    let payload = Payload::default();

    let res = Output::new().payload(&payload);
    assert!(res.is_ok())
}

#[test]
fn test_output_verify_digest() {
    let mut output = Output::new();

    let mut hasher = Hasher{};

    output.id = output.digest(&mut hasher).unwrap();
    
    let res = output.verify_digest(&mut hasher);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_output_check_digest() {
    let mut output = Output::new();

    let mut hasher = Hasher{};

    output.id = output.digest(&mut hasher).unwrap();
    
    let res = output.check_digest(&mut hasher);
    assert!(res.is_ok())
}

#[test]
fn test_output_finalize() {
    let meta = Meta::default();
    let amount = Amount::default();
    let payload = Payload::default();

    let mut output = Output::new()
                        .meta(&meta)
                        .unwrap()
                        .amount(&amount)
                        .unwrap()
                        .payload(&payload)
                        .unwrap();

    let mut hasher = Hasher{};

    let res = output.clone().finalize(&mut hasher);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    output.meta = invalid_meta;

    let res = output.finalize(&mut hasher);
    assert!(res.is_err());
}

#[test]
fn test_output_check() {
    let meta = Meta::default();
    let amount = Amount::default();
    let payload = Payload::default();

    let mut hasher = Hasher{};

    let mut output = Output::new()
                        .meta(&meta)
                        .unwrap()
                        .amount(&amount)
                        .unwrap()
                        .payload(&payload)
                        .unwrap()
                        .finalize(&mut hasher)
                        .unwrap();

    let res = output.check();
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    output.meta = invalid_meta;

    let res = output.check();
    assert!(res.is_err());
}

#[test]
fn test_output_eval() {
    let meta = Meta::default();
    let amount = Amount::default();
    let payload = Payload::new("pAyLoAd");

    let mut hasher = Hasher{};

    let mut output = Output::new()
                        .meta(&meta)
                        .unwrap()
                        .amount(&amount)
                        .unwrap()
                        .payload(&payload)
                        .unwrap()
                        .finalize(&mut hasher)
                        .unwrap();

    let mut evaluator = OutputEvaluator{};

    let res = output.eval(&PayloadEvalParams::Const, &evaluator);
    assert!(res.is_ok());

    let const_res = res.unwrap();
    assert_eq!(const_res, PayloadEvalResult::Const(payload.to_string()));

    let res = output.eval_mut(&PayloadEvalMutParams::ToUppercase, &mut evaluator);
    assert!(res.is_ok());

    let to_uppercase_res = res.unwrap();

    let uppsercase_payload = payload.to_string().to_uppercase();
    assert_eq!(to_uppercase_res, PayloadEvalMutResult::ToUppercase(uppsercase_payload.clone()));
    assert_eq!(output.payload.to_string(), uppsercase_payload)
}

#[test]
fn test_output_size() {
    let output = Output::new();

    let meta_size = output.meta.get_size();
    let output_size = output.size();

    assert_eq!(meta_size, output_size);
}

#[test]
fn test_output_json() {
    let output_a = Output::new();

    let res = output_a.to_json();
    assert!(res.is_ok());

    let output_json = res.unwrap();

    let res = Output::from_json(&output_json);
    assert!(res.is_ok());

    let output_b = res.unwrap();

    assert_eq!(output_a, output_b);
}

#[test]
fn test_output_bytes() {
    let output_a = Output::new();

    let res = output_a.to_bytes();
    assert!(res.is_ok());

    let output_bytes = res.unwrap();

    let res = Output::from_bytes(&output_bytes);
    assert!(res.is_ok());

    let output_b = res.unwrap();

    assert_eq!(output_a, output_b);
}

#[test]
fn test_output_hex() {
    let output_a = Output::new();

    let res = output_a.to_hex();
    assert!(res.is_ok());

    let output_hex = res.unwrap();

    let res = Output::from_hex(&output_hex);
    assert!(res.is_ok());

    let output_b = res.unwrap();

    assert_eq!(output_a, output_b);
}

#[test]
fn test_output_store() {
    let meta = Meta::default();
    let amount = Amount::default();
    let payload = Payload::default();

    let mut hasher = Hasher{};

    let output = Output::new()
                    .meta(&meta)
                    .unwrap()
                    .amount(&amount)
                    .unwrap()
                    .payload(&payload)
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let mut store = Store::new();
    let res = output.store_create(&mut store);
    assert!(res.is_ok());

    let res = output.store_create(&mut store);
    assert!(res.is_err());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let mut invalid_output = output.clone();
    invalid_output.meta = invalid_meta;

    let res = invalid_output.store_create(&mut store);
    assert!(res.is_err());

    let res = Output::store_lookup(&mut store, &output.id);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let unknown_id = Digest::default();

    let res = Output::store_lookup(&mut store, &unknown_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Output::store_get(&mut store, &output.id);
    assert!(res.is_ok());

    let found_output = res.unwrap();
    assert_eq!(found_output, output);

    let res = Output::store_get(&mut store, &unknown_id);
    assert!(res.is_err());

    let mut from = Some(output.id.clone());
    let mut to = Some(output.id.clone());

    let res = Output::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Output::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(output.id.clone());

    let res = Output::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = None;
    to = Some(output.id.clone());

    let res = Output::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let mut from = Some(output.id.clone());
    let mut to = Some(output.id.clone());
    let mut count = None;
    let skip = 0;

    let res = Output::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_err());

    count = Some(0);

    let res = Output::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_err());

    from = None;
    to = None;
    count = None;

    let res = Output::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![output.clone()]);

    from = Some(output.id.clone());

    let res = Output::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![output.clone()]);

    from = None;
    to = Some(output.id.clone());

    let res = Output::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = output.store_delete(&mut store);
    assert!(res.is_ok());

    let res = output.store_delete(&mut store);
    assert!(res.is_err());

    let res = Output::store_lookup(&mut store, &output.id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Output::store_get(&mut store, &output.id);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Output::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let count = None;

    let res = Output::store_list(&mut store, to.clone(), from.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = output.store_upsert(&mut store);
    assert!(res.is_ok());

    let res = Output::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    let count = None;

    let res = Output::store_list(&mut store, to, from, count, skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![output.clone()]);
}