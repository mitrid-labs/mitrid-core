use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::models::Output as BaseOutput;

use fixtures::base::eval::*;
use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::PublicKey;
use fixtures::models::Amount;
use fixtures::models::Payload;

#[allow(dead_code)]
pub type Output = BaseOutput<Digest, PublicKey, Amount, Payload>;

#[allow(dead_code)]
pub fn output_digest_cb(output: &Output, _: &()) -> Result<Digest> {
    let msg = output.to_bytes()?;
    SHA512::digest(&msg)
}

#[allow(dead_code)]
pub fn output_verify_digest_cb(output: &Output, _: &(), digest: &Digest) -> Result<bool> {
    let target = output_digest_cb(output, &())?;
    
    Ok(&target == digest)
}

#[allow(dead_code)]
pub fn output_check_digest_cb(output: &Output, _: &(), digest: &Digest) -> Result<()> {
    if !output_verify_digest_cb(output, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}

#[allow(dead_code)]
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