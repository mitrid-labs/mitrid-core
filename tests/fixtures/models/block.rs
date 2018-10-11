use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::models::Block as BaseBlock;

use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::{PublicKey, Signature};
use fixtures::models::Amount;
use fixtures::models::Payload;

#[allow(dead_code)]
pub type Block = BaseBlock<Digest, Amount, Payload, PublicKey, Signature, Payload, Payload, Payload, Payload>;

#[allow(dead_code)]
pub fn block_digest_cb(block: &Block, _: &()) -> Result<Digest> {
    let msg = block.to_bytes()?;
    SHA512::digest(&msg)
}