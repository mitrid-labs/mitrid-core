use mitrid_core::model::Coin as BaseCoin;

use fixture::crypto::Digest;
use fixture::model::Amount;

pub type Coin = BaseCoin<Digest, Amount>;