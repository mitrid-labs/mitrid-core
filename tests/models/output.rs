use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::utils::Version;
use mitrid_core::models::Meta;

use fixtures::base::eval::*;
use fixtures::crypto::PublicKey;
use fixtures::crypto::SHA512HMAC;
use fixtures::models::Amount;
use fixtures::models::Payload;
use fixtures::models::output::*;

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
fn test_output_sender() {
    let sender = PublicKey::default();
    
    let res = Output::new().sender(&sender);
    assert!(res.is_ok());
}

#[test]
fn test_output_receiver() {
    let receiver = PublicKey::default();
    
    let res = Output::new().receiver(&receiver);
    assert!(res.is_ok());
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

    output.id = output.digest(&(), &output_digest_cb).unwrap();
    
    let res = output.verify_digest(&(), &output_verify_digest_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_output_check_digest() {
    let mut output = Output::new();

    output.id = output.digest(&(), &output_digest_cb).unwrap();
    
    let res = output.check_digest(&(), &output_check_digest_cb);
    assert!(res.is_ok())
}

#[test]
fn test_output_commit() {
    let output = Output::new();

    let res = output.commit(&(), &output_commit_cb);
    assert!(res.is_ok());
}

#[test]
fn test_output_verify_commitment() {
    let output = Output::new();

    let commitment = output.commit(&(), &output_commit_cb).unwrap();
    
    let res = output.verify_commitment(&(), &commitment, &output_verify_commitment_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_output_check_commitment() {
    let output = Output::new();

    let commitment = output.commit(&(), &output_commit_cb).unwrap();
    
    let res = output.check_commitment(&(), &commitment, &output_check_commitment_cb);
    assert!(res.is_ok())
}

#[test]
fn test_output_authenticate() {
    let output = Output::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let res = output.authenticate(&key, &output_authenticate_cb);
    assert!(res.is_ok());
}

#[test]
fn test_output_verify_tag() {
    let output = Output::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let tag = output.authenticate(&key, &output_authenticate_cb).unwrap();
    
    let res = output.verify_tag(&key, &tag, &output_verify_tag_cb);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_output_check_tag() {
    let output = Output::new();

    let res = SHA512HMAC::genkey();
    assert!(res.is_ok());

    let key = res.unwrap();

    let tag = output.authenticate(&key, &output_authenticate_cb).unwrap();
    
    let res = output.check_tag(&key, &tag, &output_check_tag_cb);
    assert!(res.is_ok())
}

#[test]
fn test_output_finalize() {
    let meta = Meta::default();
    let sender = PublicKey::default();
    let receiver = PublicKey::default();
    let amount = Amount::default();
    let payload = Payload::default();

    let mut output = Output::new()
                        .meta(&meta)
                        .unwrap()
                        .sender(&sender)
                        .unwrap()
                        .receiver(&receiver)
                        .unwrap()
                        .amount(&amount)
                        .unwrap()
                        .payload(&payload)
                        .unwrap();

    let res = output.clone().finalize(&(), &output_digest_cb);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    output.meta = invalid_meta;

    let res = output.finalize(&(), &output_digest_cb);
    assert!(res.is_err());
}

#[test]
fn test_output_check() {
    let meta = Meta::default();
    let sender = PublicKey::default();
    let receiver = PublicKey::default();
    let amount = Amount::default();
    let payload = Payload::default();

    let mut output = Output::new()
                        .meta(&meta)
                        .unwrap()
                        .sender(&sender)
                        .unwrap()
                        .receiver(&receiver)
                        .unwrap()
                        .amount(&amount)
                        .unwrap()
                        .payload(&payload)
                        .unwrap()
                        .finalize(&(), &output_digest_cb)
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
    let sender = PublicKey::default();
    let receiver = PublicKey::default();
    let amount = Amount::default();
    let payload = Payload::new("pAyLoAd");

    let output = Output::new()
                    .meta(&meta)
                    .unwrap()
                    .sender(&sender)
                    .unwrap()
                    .receiver(&receiver)
                    .unwrap()
                    .amount(&amount)
                    .unwrap()
                    .payload(&payload)
                    .unwrap()
                    .finalize(&(), &output_digest_cb)
                    .unwrap();

    let res = output.eval(&EvalParams::Const, &output_eval_cb);
    assert!(res.is_ok());

    let const_res = res.unwrap();
    assert_eq!(const_res, EvalReturn::Const(payload.to_string()));

    let res = output.eval(&EvalParams::ToUppercase, &output_eval_cb);
    assert!(res.is_ok());

    let to_uppercase_res = res.unwrap();
    assert_eq!(to_uppercase_res, EvalReturn::ToUppercase(payload.to_string().to_uppercase()));
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