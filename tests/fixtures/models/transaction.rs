use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::models::Transaction as BaseTransaction;

use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::{PublicKey, Signature};
use fixtures::models::Amount;
use fixtures::models::Payload;

#[allow(dead_code)]
pub type Transaction = BaseTransaction<Digest, Amount, Payload, PublicKey, Signature, Payload, Payload>;

#[allow(dead_code)]
pub fn transaction_digest_cb(tx: &Transaction, _: &()) -> Result<Digest> {
    let msg = tx.to_bytes()?;
    SHA512::digest(&msg)
}

#[allow(dead_code)]
pub fn transaction_verify_digest_cb(tx: &Transaction, _: &(), digest: &Digest) -> Result<bool> {
    let target = transaction_digest_cb(tx, &())?;
    
    Ok(&target == digest)
}

#[allow(dead_code)]
pub fn transaction_check_digest_cb(tx: &Transaction, _: &(), digest: &Digest) -> Result<()> {
    if !transaction_verify_digest_cb(tx, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}