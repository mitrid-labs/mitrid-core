#![allow(dead_code)]

use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::models::BlockNode as BaseBlockNode;

use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::{Commitment, SHA512Commit};
use fixtures::crypto::{AuthKey, Tag, SHA512HMAC};

pub type BlockNode = BaseBlockNode<Digest>;

pub fn blocknode_digest_cb(bn: &BlockNode, _: &()) -> Result<Digest> {
    let msg = bn.to_bytes()?;
    SHA512::digest(&msg)
}

pub fn blocknode_verify_digest_cb(bn: &BlockNode, _: &(), digest: &Digest) -> Result<bool> {
    let target = blocknode_digest_cb(bn, &())?;
    
    Ok(&target == digest)
}

pub fn blocknode_check_digest_cb(bn: &BlockNode, _: &(), digest: &Digest) -> Result<()> {
    if !blocknode_verify_digest_cb(bn, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}

pub fn blocknode_commit_cb(bn: &BlockNode, _: &()) -> Result<Commitment> {
    let msg = bn.to_bytes()?;
    SHA512Commit::commit(&msg)
}

pub fn blocknode_verify_commitment_cb(bn: &BlockNode, _: &(), commitment: &Commitment) -> Result<bool> {
    let msg = bn.to_bytes()?;
    SHA512Commit::verify(&msg, commitment)
}

pub fn blocknode_check_commitment_cb(bn: &BlockNode, _: &(), commitment: &Commitment) -> Result<()> {
    let msg = bn.to_bytes()?;
    SHA512Commit::check(&msg, commitment)
}

pub fn blocknode_authenticate_cb(bn: &BlockNode, key: &AuthKey) -> Result<Commitment> {
    let msg = bn.to_bytes()?;

    SHA512HMAC::authenticate(&msg, &key)
}

pub fn blocknode_verify_tag_cb(bn: &BlockNode, key: &AuthKey, tag: &Tag) -> Result<bool> {
    let msg = bn.to_bytes()?;
    SHA512HMAC::verify(&msg, key, tag)
}

pub fn blocknode_check_tag_cb(bn: &BlockNode, key: &AuthKey, tag: &Tag) -> Result<()> {
    let msg = bn.to_bytes()?;
    SHA512HMAC::check(&msg, key, tag)
}