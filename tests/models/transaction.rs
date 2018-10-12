use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::utils::Version;
use mitrid_core::models::Meta;

use fixtures::models::Payload;
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
fn test_transaction_inputs() {}

#[test]
fn test_transaction_outputs() {}

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