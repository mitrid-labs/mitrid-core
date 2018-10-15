#![allow(dead_code)]

use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::models::Input as BaseInput;

use fixtures::base::eval::*;
use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::{SecretKey, PublicKey, Signature, Ed25519};
use fixtures::crypto::{Commitment, SHA512Commit};
use fixtures::models::Amount;
use fixtures::models::Payload;

pub type Input = BaseInput<Digest, Amount, Payload, PublicKey, Signature>;

pub fn input_digest_cb(input: &Input, _: &()) -> Result<Digest> {
    let msg = input.to_bytes()?;
    SHA512::digest(&msg)
}

pub fn input_verify_digest_cb(input: &Input, _: &(), digest: &Digest) -> Result<bool> {
    let target = input_digest_cb(input, &())?;
    
    Ok(&target == digest)
}

pub fn input_check_digest_cb(input: &Input, _: &(), digest: &Digest) -> Result<()> {
    if !input_verify_digest_cb(input, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}

pub fn input_sign_cb(input: &Input, _: &(), sk: &SecretKey) -> Result<Signature> {
    let msg = input.to_bytes()?;
    Ed25519::sign(&msg, sk)
}

pub fn input_verify_signature_cb(input: &Input, _: &(), pk: &PublicKey, sig: &Signature) -> Result<bool> {
    let msg = input.to_bytes()?;
    Ed25519::verify(&msg, pk, sig)
}

pub fn input_check_signature_cb(input: &Input, _: &(), pk: &PublicKey, sig: &Signature) -> Result<()> {
    if !input_verify_signature_cb(input, &(), pk, sig)? {
        return Err("invalid signature".into());
    }

    Ok(())
}

pub fn input_commit_cb(input: &Input, _: &()) -> Result<Commitment> {
    let msg = input.to_bytes()?;
    SHA512Commit::commit(&msg)
}

pub fn input_verify_commitment_cb(input: &Input, _: &(), commitment: &Commitment) -> Result<bool> {
    let msg = input.to_bytes()?;
    SHA512Commit::verify(&msg, commitment)
}

pub fn input_check_commitment_cb(input: &Input, _: &(), commitment: &Commitment) -> Result<()> {
    let msg = input.to_bytes()?;
    SHA512Commit::check(&msg, commitment)
}

pub fn input_eval_cb(input: &Input, params: &EvalParams) -> Result<EvalReturn> {
    input.check()?;
    params.check()?;

    let s = input.payload.to_string();

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