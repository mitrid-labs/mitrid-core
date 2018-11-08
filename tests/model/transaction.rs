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
use fixture::model::coin::*;
use fixture::model::input::*;
use fixture::model::output::*;
use fixture::model::transaction::*;
use fixture::io::store::*;

#[test]
fn test_transaction_meta() {
    let valid_meta = Meta::default();
    
    let res = Transaction::new().meta(&valid_meta);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let res = Transaction::new().meta(&invalid_meta);
    assert!(res.is_err())
}

#[test]
fn test_transaction_inputs() {
    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .finalize(&mut hasher)
                    .unwrap();

    let mut input = Input::new()
                        .meta(&Meta::default())
                        .unwrap()
                        .coin(&coin)
                        .unwrap()
                        .payload(&Payload::default())
                        .unwrap()
                        .finalize(&mut hasher)
                        .unwrap();

    let res = Transaction::new().inputs(&vec![input.clone(), input.clone()]);
    assert!(res.is_err());

    let res = Transaction::new().inputs(&vec![input.clone()]);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    input.meta = invalid_meta;

    let res = Transaction::new().inputs(&vec![input]);
    assert!(res.is_err());
}

#[test]
fn test_transaction_outputs() {
    let mut hasher = Hasher{};

    let mut output = Output::new()
                        .meta(&Meta::default())
                        .unwrap()
                        .amount(&Amount::default())
                        .unwrap()
                        .payload(&Payload::default())
                        .unwrap()
                        .finalize(&mut hasher)
                        .unwrap();

    let res = Transaction::new().outputs(&vec![output.clone(), output.clone()]);
    assert!(res.is_err());

    let res = Transaction::new().outputs(&vec![output.clone()]);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    output.meta = invalid_meta;

    let res = Transaction::new().outputs(&vec![output]);
    assert!(res.is_err());
}

#[test]
fn test_transaction_payload() {
    let payload = Payload::default();

    let res = Transaction::new().payload(&payload);
    assert!(res.is_ok())
}

#[test]
fn test_transaction_verify_digest() {
    let mut tx = Transaction::new();

    let mut hasher = Hasher{};

    tx.id = tx.digest(&mut hasher).unwrap();
    
    let res = tx.verify_digest(&mut hasher);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_transaction_check_digest() {
    let mut tx = Transaction::new();

    let mut hasher = Hasher{};

    tx.id = tx.digest(&mut hasher).unwrap();
    
    let res = tx.check_digest(&mut hasher);
    assert!(res.is_ok())
}

#[test]
fn test_transaction_finalize() {
    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .finalize(&mut hasher)
                    .unwrap();

    let input = Input::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let output = Output::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .amount(&Amount::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let payload = Payload::default();

    let mut tx = Transaction::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .inputs(&vec![input])
                    .unwrap()
                    .outputs(&vec![output])
                    .unwrap()
                    .payload(&payload)
                    .unwrap();

    let res = tx.clone().finalize(&mut hasher);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    tx.inputs[0].meta = invalid_meta;

    let res = tx.finalize(&mut hasher);
    assert!(res.is_err());
}

#[test]
fn test_transaction_check() {
    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .finalize(&mut hasher)
                    .unwrap();

    let input = Input::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let output = Output::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .amount(&Amount::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let payload = Payload::default();

    let mut tx = Transaction::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .inputs(&vec![input])
                    .unwrap()
                    .outputs(&vec![output])
                    .unwrap()
                    .payload(&payload)
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let res = tx.check();
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let invalid_meta = Meta::default();
    tx.meta = invalid_meta;

    let res = tx.check();
    assert!(res.is_err());
}

#[test]
fn test_transaction_eval() {
    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .finalize(&mut hasher)
                    .unwrap();

    let input = Input::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let output = Output::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .amount(&Amount::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let payload = Payload::default();

    let mut tx = Transaction::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .inputs(&vec![input])
                    .unwrap()
                    .outputs(&vec![output])
                    .unwrap()
                    .payload(&payload)
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let mut evaluator = TransactionEvaluator{};

    let res = tx.eval(&PayloadEvalParams::Const, &evaluator);
    assert!(res.is_ok());

    let const_res = res.unwrap();
    assert_eq!(const_res, PayloadEvalResult::Const(payload.to_string()));

    let res = tx.eval_mut(&PayloadEvalMutParams::ToUppercase, &mut evaluator);
    assert!(res.is_ok());

    let to_uppercase_res = res.unwrap();

    let uppsercase_payload = payload.to_string().to_uppercase();
    assert_eq!(to_uppercase_res, PayloadEvalMutResult::ToUppercase(uppsercase_payload.clone()));
    assert_eq!(tx.payload.to_string(), uppsercase_payload)
}

#[test]
fn test_transaction_size() {
    let tx = Transaction::new();

    let meta_size = tx.meta.get_size();
    let tx_size = tx.size();

    assert_eq!(meta_size, tx_size);
}

#[test]
fn test_transaction_json() {
    let tx_a = Transaction::new();

    let res = tx_a.to_json();
    assert!(res.is_ok());

    let tx_json = res.unwrap();

    let res = Transaction::from_json(&tx_json);
    assert!(res.is_ok());

    let tx_b = res.unwrap();

    assert_eq!(tx_a, tx_b);
}

#[test]
fn test_transaction_bytes() {
    let tx_a = Transaction::new();

    let res = tx_a.to_bytes();
    assert!(res.is_ok());

    let tx_bytes = res.unwrap();

    let res = Transaction::from_bytes(&tx_bytes);
    assert!(res.is_ok());

    let tx_b = res.unwrap();

    assert_eq!(tx_a, tx_b);
}

#[test]
fn test_transaction_hex() {
    let tx_a = Transaction::new();

    let res = tx_a.to_hex();
    assert!(res.is_ok());

    let tx_hex = res.unwrap();

    let res = Transaction::from_hex(&tx_hex);
    assert!(res.is_ok());

    let tx_b = res.unwrap();

    assert_eq!(tx_a, tx_b);
}

#[test]
fn test_transaction_store() {
    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .finalize(&mut hasher)
                    .unwrap();

    let input = Input::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let output = Output::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .amount(&Amount::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let payload = Payload::default();

    let tx = Transaction::new()
                .meta(&Meta::default())
                .unwrap()
                .inputs(&vec![input])
                .unwrap()
                .outputs(&vec![output])
                .unwrap()
                .payload(&payload)
                .unwrap()
                .finalize(&mut hasher)
                .unwrap();

    let mut store = Store::new();
    let res = tx.store_create(&mut store);
    assert!(res.is_ok());

    let res = tx.store_create(&mut store);
    assert!(res.is_err());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let invalid_meta = Meta::default();
    
    let mut invalid_tx = tx.clone();
    invalid_tx.meta = invalid_meta;

    let res = invalid_tx.store_create(&mut store);
    assert!(res.is_err());

    let res = Transaction::store_lookup(&mut store, &tx.id);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let unknown_id = Digest::default();

    let res = Transaction::store_lookup(&mut store, &unknown_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Transaction::store_get(&mut store, &tx.id);
    assert!(res.is_ok());

    let found_tx = res.unwrap();
    assert_eq!(found_tx, tx);

    let res = Transaction::store_get(&mut store, &unknown_id);
    assert!(res.is_err());

    let mut from = Some(tx.id.clone());
    let mut to = Some(tx.id.clone());

    let res = Transaction::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Transaction::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(tx.id.clone());

    let res = Transaction::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = None;
    to = Some(tx.id.clone());

    let res = Transaction::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let mut from = Some(tx.id.clone());
    let mut to = Some(tx.id.clone());
    let mut count = None;
    let skip = 0;

    let res = Transaction::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_err());

    count = Some(0);

    let res = Transaction::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_err());

    from = None;
    to = None;
    count = None;

    let res = Transaction::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![tx.clone()]);

    from = Some(tx.id.clone());

    let res = Transaction::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![tx.clone()]);

    from = None;
    to = Some(tx.id.clone());

    let res = Transaction::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = tx.store_delete(&mut store);
    assert!(res.is_ok());

    let res = tx.store_delete(&mut store);
    assert!(res.is_err());

    let res = Transaction::store_lookup(&mut store, &coin.id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Transaction::store_get(&mut store, &coin.id);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Transaction::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let count = None;

    let res = Transaction::store_list(&mut store, to.clone(), from.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = tx.store_upsert(&mut store);
    assert!(res.is_ok());

    let res = Transaction::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    let count = None;

    let res = Transaction::store_list(&mut store, to, from, count, skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![tx.clone()]);
}