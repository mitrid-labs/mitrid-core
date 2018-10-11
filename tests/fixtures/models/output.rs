use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::models::Output as BaseOutput;

use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::PublicKey;
use fixtures::models::Amount;
use fixtures::models::Payload;

#[allow(dead_code)]
pub type Output = BaseOutput<Digest, PublicKey, Amount, Payload>;

#[allow(dead_code)]
pub fn output_digest_cb(output: &Output, _: &()) -> Result<Digest> {
    let msg = output.to_bytes()?;
    SHA512::digest(&msg)
}

#[allow(dead_code)]
pub fn output_verify_digest_cb(output: &Output, _: &(), digest: &Digest) -> Result<bool> {
    let target = output_digest_cb(output, &())?;
    
    Ok(&target == digest)
}

#[allow(dead_code)]
pub fn output_check_digest_cb(output: &Output, _: &(), digest: &Digest) -> Result<()> {
    if !output_verify_digest_cb(output, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}