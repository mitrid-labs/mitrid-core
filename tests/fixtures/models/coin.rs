use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::models::Coin as BaseCoin;

use fixtures::crypto::{Digest, SHA512};
use fixtures::models::Amount;

pub type Coin = BaseCoin<Digest, Amount>;

pub fn coin_digest_cb(coin: &Coin, _: &()) -> Result<Digest> {
    let msg = coin.to_bytes()?;
    SHA512::digest(&msg)
}

pub fn coin_verify_digest_cb(coin: &Coin, _: &(), digest: &Digest) -> Result<bool> {
    let target = coin_digest_cb(coin, &())?;
    
    Ok(&target == digest)
}

pub fn coin_check_digest_cb(coin: &Coin, _: &(), digest: &Digest) -> Result<()> {
    if !coin_verify_digest_cb(coin, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}