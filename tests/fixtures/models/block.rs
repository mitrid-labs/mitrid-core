use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::models::Block as BaseBlock;

use fixtures::base::eval::*;
use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::{PublicKey, Signature};
use fixtures::crypto::{Proof, HashCash};
use fixtures::crypto::{Commitment, SHA512Commit};
use fixtures::crypto::{AuthKey, Tag, SHA512HMAC};
use fixtures::models::Amount;
use fixtures::models::Payload;

pub type Block = BaseBlock<Digest, Amount, Payload, PublicKey, Signature, Payload, Payload, Payload, Proof>;

pub fn block_digest_cb(block: &Block, _: &()) -> Result<Digest> {
    let msg = block.to_bytes()?;
    SHA512::digest(&msg)
}

pub fn block_verify_digest_cb(block: &Block, _: &(), digest: &Digest) -> Result<bool> {
    let target = block_digest_cb(block, &())?;
    
    Ok(&target == digest)
}

pub fn block_check_digest_cb(block: &Block, _: &(), digest: &Digest) -> Result<()> {
    if !block_verify_digest_cb(block, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}

pub fn block_prove_cb(block: &Block, bits: &Option<u32>) -> Result<Proof> {
    let msg = block.to_bytes()?;
    HashCash::prove(&msg, bits.unwrap_or(0))
}

pub fn block_verify_proof_cb(block: &Block, bits: &Option<u32>, proof: &Proof) -> Result<bool> {
    let msg = block.to_bytes()?;
    HashCash::verify(&msg, bits.unwrap_or(0), proof)
}

pub fn block_check_proof_cb(block: &Block, bits: &Option<u32>, proof: &Proof) -> Result<()> {
    if !block_verify_proof_cb(block, bits, proof)? {
        return Err("invalid proof".into());
    }

    Ok(())
}

pub fn block_commit_cb(block: &Block, _: &()) -> Result<Commitment> {
    let msg = block.to_bytes()?;
    SHA512Commit::commit(&msg)
}

pub fn block_verify_commitment_cb(block: &Block, _: &(), commitment: &Commitment) -> Result<bool> {
    let msg = block.to_bytes()?;
    SHA512Commit::verify(&msg, commitment)
}

pub fn block_check_commitment_cb(block: &Block, _: &(), commitment: &Commitment) -> Result<()> {
    let msg = block.to_bytes()?;
    SHA512Commit::check(&msg, commitment)
}

pub fn block_authenticate_cb(block: &Block, key: &AuthKey) -> Result<Commitment> {
    let msg = block.to_bytes()?;

    SHA512HMAC::authenticate(&msg, &key)
}

pub fn block_verify_tag_cb(block: &Block, key: &AuthKey, tag: &Tag) -> Result<bool> {
    let msg = block.to_bytes()?;
    SHA512HMAC::verify(&msg, key, tag)
}

pub fn block_check_tag_cb(block: &Block, key: &AuthKey, tag: &Tag) -> Result<()> {
    let msg = block.to_bytes()?;
    SHA512HMAC::check(&msg, key, tag)
}

pub fn block_eval_cb(block: &Block, params: &EvalParams) -> Result<EvalReturn> {
    block.check()?;
    params.check()?;

    let s = block.payload.to_string();

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