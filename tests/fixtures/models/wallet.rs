use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::models::Wallet as BaseWallet;

use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::{SecretKey, PublicKey, Signature};
use fixtures::models::Payload;

#[allow(dead_code)]
pub type Wallet = BaseWallet<Digest, SecretKey, PublicKey, Signature, Payload>;

#[allow(dead_code)]
pub fn wallet_digest_cb(wallet: &Wallet, _: &()) -> Result<Digest> {
    let msg = wallet.to_bytes()?;
    SHA512::digest(&msg)
}

#[allow(dead_code)]
pub fn wallet_verify_digest_cb(wallet: &Wallet, _: &(), digest: &Digest) -> Result<bool> {
    let target = wallet_digest_cb(wallet, &())?;
    
    Ok(&target == digest)
}

#[allow(dead_code)]
pub fn wallet_check_digest_cb(wallet: &Wallet, _: &(), digest: &Digest) -> Result<()> {
    if !wallet_verify_digest_cb(wallet, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}