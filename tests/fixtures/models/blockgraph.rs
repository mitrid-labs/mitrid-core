use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::models::BlockGraph as BaseBlockGraph;

use fixtures::base::eval::*;
use fixtures::crypto::{Digest, SHA512};
use fixtures::models::Payload;

#[allow(dead_code)]
pub type BlockGraph = BaseBlockGraph<Digest, Payload>;

#[allow(dead_code)]
pub fn blockgraph_digest_cb(bg: &BlockGraph, _: &()) -> Result<Digest> {
    let msg = bg.to_bytes()?;
    SHA512::digest(&msg)
}

#[allow(dead_code)]
pub fn blockgraph_verify_digest_cb(bg: &BlockGraph, _: &(), digest: &Digest) -> Result<bool> {
    let target = blockgraph_digest_cb(bg, &())?;
    
    Ok(&target == digest)
}

#[allow(dead_code)]
pub fn blockgraph_check_digest_cb(bg: &BlockGraph, _: &(), digest: &Digest) -> Result<()> {
    if !blockgraph_verify_digest_cb(bg, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}

#[allow(dead_code)]
pub fn blockgraph_eval_cb(bg: &BlockGraph, params: &EvalParams) -> Result<EvalReturn> {
    bg.check()?;
    params.check()?;

    let s = bg.payload.to_string();

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