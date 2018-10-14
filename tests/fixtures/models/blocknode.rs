use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::models::BlockNode as BaseBlockNode;

use fixtures::crypto::{Digest, SHA512};

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