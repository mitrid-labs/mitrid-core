use mitrid_core::models::Input as BaseInput;

use fixtures::crypto::Digest;
use fixtures::crypto::{PublicKey, Signature};
use fixtures::models::Amount;
use fixtures::models::Payload;

#[allow(dead_code)]
pub type Input = BaseInput<Digest, Amount, Payload, PublicKey, Signature>;