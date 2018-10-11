use mitrid_core::models::BlockGraph as BaseBlockGraph;

use fixtures::crypto::Digest;
use fixtures::models::Payload;

#[allow(dead_code)]
pub type BlockGraph = BaseBlockGraph<Digest, Payload>;