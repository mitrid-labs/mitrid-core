//! # Commit
//!
//! `commit` is the module providing the trait used to implement a cryptographic commitment scheme.

use base::Result;
use base::ConstantSize;
use base::Datable;

/// Trait used by types that implements cryptographic commitment.
pub trait Commit<C>
    where   C: Datable + ConstantSize,
            Self: 'static + Sized
{
    /// Commits cryptographically a message, returning a commitment.
    fn commit(&mut self, msg: &[u8]) -> Result<C>;

    /// Verifies a commitment against a message.
    fn verify(&mut self, msg: &[u8], commitment: &C) -> Result<bool>;

    /// Checks a commitment against a message.
    fn check(&mut self, msg: &[u8], commitment: &C) -> Result<()>;
}