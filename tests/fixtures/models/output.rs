use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::models::Output as BaseOutput;

use fixtures::base::eval::*;
use fixtures::base::Payload;
use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::PublicKey;
use fixtures::crypto::{Commitment, SHA512Commit};
use fixtures::crypto::{AuthKey, Tag, SHA512HMAC};
use fixtures::models::Amount;

pub type Output = BaseOutput<Digest, PublicKey, Amount, Payload>;

pub fn output_digest_cb(output: &Output, _: &()) -> Result<Digest> {
    let msg = output.to_bytes()?;
    SHA512::digest(&msg)
}

pub fn output_verify_digest_cb(output: &Output, _: &(), digest: &Digest) -> Result<bool> {
    let target = output_digest_cb(output, &())?;
    
    Ok(&target == digest)
}

pub fn output_check_digest_cb(output: &Output, _: &(), digest: &Digest) -> Result<()> {
    if !output_verify_digest_cb(output, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}

pub fn output_commit_cb(output: &Output, _: &()) -> Result<Commitment> {
    let msg = output.to_bytes()?;
    SHA512Commit::commit(&msg)
}

pub fn output_verify_commitment_cb(output: &Output, _: &(), commitment: &Commitment) -> Result<bool> {
    let msg = output.to_bytes()?;
    SHA512Commit::verify(&msg, commitment)
}

pub fn output_check_commitment_cb(output: &Output, _: &(), commitment: &Commitment) -> Result<()> {
    let msg = output.to_bytes()?;
    SHA512Commit::check(&msg, commitment)
}

pub fn output_authenticate_cb(output: &Output, key: &AuthKey) -> Result<Commitment> {
    let msg = output.to_bytes()?;

    SHA512HMAC::authenticate(&msg, &key)
}

pub fn output_verify_tag_cb(output: &Output, key: &AuthKey, tag: &Tag) -> Result<bool> {
    let msg = output.to_bytes()?;
    SHA512HMAC::verify(&msg, key, tag)
}

pub fn output_check_tag_cb(output: &Output, key: &AuthKey, tag: &Tag) -> Result<()> {
    let msg = output.to_bytes()?;
    SHA512HMAC::check(&msg, key, tag)
}

pub fn output_eval_cb(output: &Output, params: &EvalParams) -> Result<EvalReturn> {
    output.check()?;
    params.check()?;

    let s = output.payload.to_string();

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