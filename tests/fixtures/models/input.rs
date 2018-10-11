use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::models::Input as BaseInput;

use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::{PublicKey, Signature};
use fixtures::models::Amount;
use fixtures::models::Payload;

#[allow(dead_code)]
pub type Input = BaseInput<Digest, Amount, Payload, PublicKey, Signature>;

#[allow(dead_code)]
pub fn input_digest_cb(input: &Input, _: &()) -> Result<Digest> {
    let msg = input.to_bytes()?;
    SHA512::digest(&msg)
}