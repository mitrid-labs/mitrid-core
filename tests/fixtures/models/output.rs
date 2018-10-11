use mitrid_core::models::Output as BaseOutput;

use fixtures::crypto::Digest;
use fixtures::crypto::PublicKey;
use fixtures::models::Amount;
use fixtures::models::Payload;

#[allow(dead_code)]
pub type Output = BaseOutput<Digest, PublicKey, Amount, Payload>;