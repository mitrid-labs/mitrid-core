use mitrid_core::models::Transaction as BaseTransaction;

use fixtures::crypto::Digest;
use fixtures::crypto::{PublicKey, Signature};
use fixtures::models::Amount;
use fixtures::models::Payload;

#[allow(dead_code)]
pub type Transaction = BaseTransaction<Digest, Amount, Payload, PublicKey, Signature, Payload, Payload>;