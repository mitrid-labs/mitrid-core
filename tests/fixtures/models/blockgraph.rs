use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::models::BlockGraph as BaseBlockGraph;

use fixtures::base::eval::*;
use fixtures::base::Payload;
use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::{Commitment, SHA512Commit};
use fixtures::crypto::{AuthKey, Tag, SHA512HMAC};

pub type BlockGraph = BaseBlockGraph<Digest, Payload>;

pub fn blockgraph_digest_cb(bg: &BlockGraph, _: &()) -> Result<Digest> {
    let msg = bg.to_bytes()?;
    SHA512::digest(&msg)
}

pub fn blockgraph_verify_digest_cb(bg: &BlockGraph, _: &(), digest: &Digest) -> Result<bool> {
    let target = blockgraph_digest_cb(bg, &())?;
    
    Ok(&target == digest)
}

pub fn blockgraph_check_digest_cb(bg: &BlockGraph, _: &(), digest: &Digest) -> Result<()> {
    if !blockgraph_verify_digest_cb(bg, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}

pub fn blockgraph_commit_cb(bg: &BlockGraph, _: &()) -> Result<Commitment> {
    let msg = bg.to_bytes()?;
    SHA512Commit::commit(&msg)
}

pub fn blockgraph_verify_commitment_cb(bg: &BlockGraph, _: &(), commitment: &Commitment) -> Result<bool> {
    let msg = bg.to_bytes()?;
    SHA512Commit::verify(&msg, commitment)
}

pub fn blockgraph_check_commitment_cb(bg: &BlockGraph, _: &(), commitment: &Commitment) -> Result<()> {
    let msg = bg.to_bytes()?;
    SHA512Commit::check(&msg, commitment)
}

pub fn blockgraph_authenticate_cb(bg: &BlockGraph, key: &AuthKey) -> Result<Commitment> {
    let msg = bg.to_bytes()?;

    SHA512HMAC::authenticate(&msg, &key)
}

pub fn blockgraph_verify_tag_cb(bg: &BlockGraph, key: &AuthKey, tag: &Tag) -> Result<bool> {
    let msg = bg.to_bytes()?;
    SHA512HMAC::verify(&msg, key, tag)
}

pub fn blockgraph_check_tag_cb(bg: &BlockGraph, key: &AuthKey, tag: &Tag) -> Result<()> {
    let msg = bg.to_bytes()?;
    SHA512HMAC::check(&msg, key, tag)
}

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