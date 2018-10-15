#![allow(dead_code)]

use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::models::Transaction as BaseTransaction;

use fixtures::base::eval::*;
use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::{PublicKey, Signature};
use fixtures::crypto::{Commitment, SHA512Commit};
use fixtures::crypto::{AuthKey, Tag, SHA512HMAC};
use fixtures::models::Amount;
use fixtures::models::Payload;

pub type Transaction = BaseTransaction<Digest, Amount, Payload, PublicKey, Signature, Payload, Payload>;

pub fn transaction_digest_cb(tx: &Transaction, _: &()) -> Result<Digest> {
    let msg = tx.to_bytes()?;
    SHA512::digest(&msg)
}

pub fn transaction_verify_digest_cb(tx: &Transaction, _: &(), digest: &Digest) -> Result<bool> {
    let target = transaction_digest_cb(tx, &())?;
    
    Ok(&target == digest)
}

pub fn transaction_check_digest_cb(tx: &Transaction, _: &(), digest: &Digest) -> Result<()> {
    if !transaction_verify_digest_cb(tx, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}

pub fn transaction_commit_cb(tx: &Transaction, _: &()) -> Result<Commitment> {
    let msg = tx.to_bytes()?;
    SHA512Commit::commit(&msg)
}

pub fn transaction_verify_commitment_cb(tx: &Transaction, _: &(), commitment: &Commitment) -> Result<bool> {
    let msg = tx.to_bytes()?;
    SHA512Commit::verify(&msg, commitment)
}

pub fn transaction_check_commitment_cb(tx: &Transaction, _: &(), commitment: &Commitment) -> Result<()> {
    let msg = tx.to_bytes()?;
    SHA512Commit::check(&msg, commitment)
}

pub fn transaction_authenticate_cb(tx: &Transaction, key: &AuthKey) -> Result<Commitment> {
    let msg = tx.to_bytes()?;

    SHA512HMAC::authenticate(&msg, &key)
}

pub fn transaction_verify_tag_cb(tx: &Transaction, key: &AuthKey, tag: &Tag) -> Result<bool> {
    let msg = tx.to_bytes()?;
    SHA512HMAC::verify(&msg, key, tag)
}

pub fn transaction_check_tag_cb(tx: &Transaction, key: &AuthKey, tag: &Tag) -> Result<()> {
    let msg = tx.to_bytes()?;
    SHA512HMAC::check(&msg, key, tag)
}

pub fn transaction_eval_cb(tx: &Transaction, params: &EvalParams) -> Result<EvalReturn> {
    tx.check()?;
    params.check()?;

    let s = tx.payload.to_string();

    match params {
        &EvalParams::Const => {
            let res = EvalReturn::Const(s);
            Ok(res)
        },
        &EvalParams::IsEmpty => {
            let res = EvalReturn::IsEmpty(s.is_empty());
            Ok(res)
        },
        &EvalParams::ToUppercase => {
            let res = EvalReturn::ToUppercase(s.to_uppercase());
            Ok(res)
        },
        &EvalParams::ToLowercase => {
            let res = EvalReturn::ToLowercase(s.to_lowercase());
            Ok(res)
        },
    }
}