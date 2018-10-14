use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::models::Transaction as BaseTransaction;

use fixtures::base::eval::*;
use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::{PublicKey, Signature};
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