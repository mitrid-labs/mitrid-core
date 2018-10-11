use mitrid_core::models::Block as BaseBlock;

use fixtures::crypto::Digest;
use fixtures::crypto::{PublicKey, Signature};
use fixtures::models::Amount;
use fixtures::models::Payload;

#[allow(dead_code)]
pub type Block = BaseBlock<Digest, Amount, Payload, PublicKey, Signature, Payload, Payload, Payload, Payload>;