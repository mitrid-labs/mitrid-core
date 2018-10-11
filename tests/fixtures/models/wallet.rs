use mitrid_core::models::Wallet as BaseWallet;

use fixtures::crypto::Digest;
use fixtures::crypto::{SecretKey, PublicKey, Signature};
use fixtures::models::Payload;

#[allow(dead_code)]
pub type Wallet = BaseWallet<Digest, SecretKey, PublicKey, Signature, Payload>;