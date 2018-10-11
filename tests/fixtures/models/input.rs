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

#[allow(dead_code)]
pub fn input_verify_digest_cb(input: &Input, _: &(), digest: &Digest) -> Result<bool> {
    let target = input_digest_cb(input, &())?;
    
    Ok(&target == digest)
}

#[allow(dead_code)]
pub fn input_check_digest_cb(input: &Input, _: &(), digest: &Digest) -> Result<()> {
    if !input_verify_digest_cb(input, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}