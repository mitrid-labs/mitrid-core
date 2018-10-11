use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::models::BlockGraph as BaseBlockGraph;

use fixtures::crypto::{Digest, SHA512};
use fixtures::models::Payload;

#[allow(dead_code)]
pub type BlockGraph = BaseBlockGraph<Digest, Payload>;

#[allow(dead_code)]
pub fn blockgraph_digest_cb(bg: &BlockGraph, _: &()) -> Result<Digest> {
    let msg = bg.to_bytes()?;
    SHA512::digest(&msg)
}