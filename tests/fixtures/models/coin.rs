use mitrid_core::models::Coin as BaseCoin;

use fixtures::crypto::Digest;
use fixtures::models::Amount;

#[allow(dead_code)]
pub type Coin = BaseCoin<Digest, Amount>;