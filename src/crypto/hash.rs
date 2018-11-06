//! # Hash
//!
//! `hash` is the module providing the trait used to implement cryptographic hashing.

use base::Result;
use base::ConstantSize;
use base::Datable;

/// Trait used by types that implements cryptographic digest.
pub trait Hash<D>
    where   D: Datable + ConstantSize,
            Self: 'static + Sized
{
    /// Hashes cryptographically a message, returning a digest.
    fn digest(&mut self, msg: &[u8]) -> Result<D>;

    /// Verifies a digest against a message.
    fn verify(&mut self, msg: &[u8], digest: &D) -> Result<bool>;

    /// Checks a digest against a message.
    fn check(&mut self, msg: &[u8], digest: &D) -> Result<()>;
}