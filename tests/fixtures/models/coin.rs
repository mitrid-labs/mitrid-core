use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::models::Coin as BaseCoin;

use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::{Commitment, SHA512Commit};
use fixtures::crypto::{AuthKey, Tag, SHA512HMAC};
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

pub fn coin_commit_cb(coin: &Coin, _: &()) -> Result<Commitment> {
    let msg = coin.to_bytes()?;
    SHA512Commit::commit(&msg)
}

pub fn coin_verify_commitment_cb(coin: &Coin, _: &(), commitment: &Commitment) -> Result<bool> {
    let msg = coin.to_bytes()?;
    SHA512Commit::verify(&msg, commitment)
}

pub fn coin_check_commitment_cb(coin: &Coin, _: &(), commitment: &Commitment) -> Result<()> {
    let msg = coin.to_bytes()?;
    SHA512Commit::check(&msg, commitment)
}

pub fn coin_authenticate_cb(coin: &Coin, key: &AuthKey) -> Result<Commitment> {
    let msg = coin.to_bytes()?;
    
    SHA512HMAC::authenticate(&msg, &key)
}

pub fn coin_verify_tag_cb(coin: &Coin, key: &AuthKey, tag: &Tag) -> Result<bool> {
    let msg = coin.to_bytes()?;
    SHA512HMAC::verify(&msg, key, tag)
}

pub fn coin_check_tag_cb(coin: &Coin, key: &AuthKey, tag: &Tag) -> Result<()> {
    let msg = coin.to_bytes()?;
    SHA512HMAC::check(&msg, key, tag)
}