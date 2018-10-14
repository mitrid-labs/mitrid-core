//! # Hash
//!
//! `hash` is the module providing the trait used to implement cryptographic hashing.

use base::Result;
use base::ConstantSize;
use base::Datable;

/// Trait implemented by types that can be cryptographically hashed.
pub trait Hashable<P, D>
    where   P: Datable,
            D: Datable + ConstantSize,
            Self: Datable
{
    /// Hashes cryptographically the implementor using `Datable` params and a callback.
    /// Returns a cryptographic digest.
    fn digest_cb(&self, params: &P, cb: &Fn(&Self, &P) -> Result<D>) -> Result<D> {
        cb(self, params)
    }

    /// Verifies the cryptographic digest against the implementor digest.
    fn verify_digest_cb(&self, params: &P, digest: &D, cb: &Fn(&Self, &P, &D) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, digest)
    }

    /// Verifies the cryptographic digest against the implementor digest.
    fn check_digest_cb(&self, params: &P, digest: &D, cb: &Fn(&Self, &P, &D) -> Result<()>)
        -> Result<()>
    {
        cb(self, params, digest)
    }
}