use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::utils::Version;
use mitrid_core::models::Meta;

use fixtures::base::eval::*;
use fixtures::base::Payload;
use fixtures::crypto::{PublicKey, Ed25519};
use fixtures::crypto::SHA512HMAC;
use fixtures::models::Amount;
use fixtures::models::coin::*;
use fixtures::models::input::*;
use fixtures::models::output::*;
use fixtures::models::transaction::*;

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
    let coin = Coin::new()
                    .finalize(&(), &coin_digest_cb)
                    .unwrap();

    let (pk, sk) = Ed25519::keypair(None).unwrap();

    let mut input = Input::new()
                        .meta(&Meta::default())
                        .unwrap()
                        .coin(&coin)
                        .unwrap()
                        .payload(&Payload::default())
                        .unwrap()
                        .sign(&(), &sk, &pk, &input_sign_cb)
                        .unwrap()
                        .finalize(&(), &input_digest_cb)
                        .unwrap();

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
    let mut output = Output::new()
                        .meta(&Meta::default())
                        .unwrap()
                        .sender(&PublicKey::default())
                        .unwrap()
                        .receiver(&PublicKey::default())
                        .unwrap()
                        .amount(&Amount::default())
                        .unwrap()
                        .payload(&Payload::default())
                        .unwrap()
                        .finalize(&(), &output_digest_cb)
                        .unwrap();

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

    tx.id = tx.digest(&(), &transaction_digest_cb).unwrap();
    
    let res = tx.verify_digest(&(), &transaction_verify_digest_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_transaction_check_digest() {
    let mut tx = Transaction::new();

    tx.id = tx.digest(&(), &transaction_digest_cb).unwrap();
    
    let res = tx.check_digest(&(), &transaction_check_digest_cb);
    assert!(res.is_ok())
}

#[test]
fn test_transaction_commit() {
    let tx = Transaction::new();

    let res = tx.commit(&(), &transaction_commit_cb);
    assert!(res.is_ok());
}

#[test]
fn test_transaction_verify_commitment() {
    let tx = Transaction::new();

    let commitment = tx.commit(&(), &transaction_commit_cb).unwrap();
    
    let res = tx.verify_commitment(&(), &commitment, &transaction_verify_commitment_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_transaction_check_commitment() {
    let transaction = Transaction::new();

    let commitment = transaction.commit(&(), &transaction_commit_cb).unwrap();
    
    let res = transaction.check_commitment(&(), &commitment, &transaction_check_commitment_cb);
    assert!(res.is_ok())
}

#[test]
fn test_transaction_authenticate() {
    let tx = Transaction::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let res = tx.authenticate(&key, &transaction_authenticate_cb);
    assert!(res.is_ok());
}

#[test]
fn test_transaction_verify_tag() {
    let tx = Transaction::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let tag = tx.authenticate(&key, &transaction_authenticate_cb).unwrap();
    
    let res = tx.verify_tag(&key, &tag, &transaction_verify_tag_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_transaction_check_tag() {
    let tx = Transaction::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let tag = tx.authenticate(&key, &transaction_authenticate_cb).unwrap();
    
    let res = tx.check_tag(&key, &tag, &transaction_check_tag_cb);
    assert!(res.is_ok())
}

#[test]
fn test_transaction_finalize() {
    let coin = Coin::new()
                    .finalize(&(), &coin_digest_cb)
                    .unwrap();

    let (pk, sk) = Ed25519::keypair(None).unwrap();

    let input = Input::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .sign(&(), &sk, &pk, &input_sign_cb)
                    .unwrap()
                    .finalize(&(), &input_digest_cb)
                    .unwrap();

    let output = Output::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .sender(&PublicKey::default())
                    .unwrap()
                    .receiver(&PublicKey::default())
                    .unwrap()
                    .amount(&Amount::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&(), &output_digest_cb)
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

    let res = tx.clone().finalize(&(), &transaction_digest_cb);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    tx.inputs[0].meta = invalid_meta;

    let res = tx.finalize(&(), &transaction_digest_cb);
    assert!(res.is_err());
}

#[test]
fn test_transaction_check() {
    let coin = Coin::new()
                    .finalize(&(), &coin_digest_cb)
                    .unwrap();

    let (pk, sk) = Ed25519::keypair(None).unwrap();

    let input = Input::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .sign(&(), &sk, &pk, &input_sign_cb)
                    .unwrap()
                    .finalize(&(), &input_digest_cb)
                    .unwrap();

    let output = Output::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .sender(&PublicKey::default())
                    .unwrap()
                    .receiver(&PublicKey::default())
                    .unwrap()
                    .amount(&Amount::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&(), &output_digest_cb)
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
                    .finalize(&(), &transaction_digest_cb)
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
    let coin = Coin::new()
                    .finalize(&(), &coin_digest_cb)
                    .unwrap();

    let (pk, sk) = Ed25519::keypair(None).unwrap();

    let input = Input::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .sign(&(), &sk, &pk, &input_sign_cb)
                    .unwrap()
                    .finalize(&(), &input_digest_cb)
                    .unwrap();

    let output = Output::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .sender(&PublicKey::default())
                    .unwrap()
                    .receiver(&PublicKey::default())
                    .unwrap()
                    .amount(&Amount::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&(), &output_digest_cb)
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
                .finalize(&(), &transaction_digest_cb)
                .unwrap();

    let res = tx.eval(&EvalParams::Const, &transaction_eval_cb);
    assert!(res.is_ok());

    let const_res = res.unwrap();
    assert_eq!(const_res, EvalReturn::Const(payload.to_string()));

    let res = tx.eval(&EvalParams::ToUppercase, &transaction_eval_cb);
    assert!(res.is_ok());

    let to_uppercase_res = res.unwrap();
    assert_eq!(to_uppercase_res, EvalReturn::ToUppercase(payload.to_string().to_uppercase()));
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