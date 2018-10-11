use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::models::Coin as BaseCoin;

use fixtures::crypto::{Digest, SHA512};
use fixtures::models::Amount;

#[allow(dead_code)]
pub type Coin = BaseCoin<Digest, Amount>;

#[allow(dead_code)]
pub fn coin_digest_cb(coin: &Coin, _: &()) -> Result<Digest> {
    let msg = coin.to_bytes()?;
    SHA512::digest(&msg)
}