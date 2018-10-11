use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::models::BlockNode as BaseBlockNode;

use fixtures::crypto::{Digest, SHA512};
use fixtures::models::Payload;

#[allow(dead_code)]
pub type BlockNode = BaseBlockNode<Digest>;

#[allow(dead_code)]
pub fn blocknode_digest_cb(bn: &BlockNode, _: &()) -> Result<Digest> {
    let msg = bn.to_bytes()?;
    SHA512::digest(&msg)
}