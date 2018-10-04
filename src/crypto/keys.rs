//! # Keys
//!
//! `keys` is the module providing the traits used to implement cryptographic key generation.

use base::Result;
use base::FixedSize;
use base::Datable;

/// Trait implemented by types that can generate cryptographic keys.
pub trait Key<P, K>
    where   P: Datable,
            K: Datable + FixedSize,
            Self: 'static + Sized
{
    /// Generates a cryptographic key using `Datable` params and a callback.
    fn generate_key_cb(&self, params: &P, cb: &Fn(&Self, &P) -> Result<K>) -> Result<K> {
        cb(self, params)
    }
}

/// Trait implemented by types that can generate cryptographic key pairs.
pub trait KeyPair<P, Sk, Pk>
    where   P: Datable,
            Sk: Datable + FixedSize,
            Pk: Datable + FixedSize,
            Self: 'static + Sized
{
    /// Generates a cryptographic key pair using `Datable` params and a callback.
    fn generate_keypair_cb(&self, params: &P, cb: &Fn(&Self, &P) -> Result<(Pk, Sk)>)
        -> Result<(Pk, Sk)>
    {
        cb(self, params)
    }
}