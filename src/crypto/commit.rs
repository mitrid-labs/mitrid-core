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
    fn check(&mut self, msg: &[u8], commitment: &C) -> Result<()> {
        if !self.verify(msg, commitment)? {
            return Err(format!("invalid commitment"));
        }

        Ok(())
    }
}

/// Trait used by types that can be cryptographically committed.
pub trait Committable<P, C>
    where   P: Datable,
            C: Datable + ConstantSize,
            Self: 'static + Sized
{
    /// Commits cryptographically the implementor using `Datable` params and a callback.
    /// Returns a commitment.
    fn commit_cb(&self, params: &P, cb: &Fn(&Self, &P) -> Result<C>) -> Result<C> {
        cb(self, params)
    }

    /// Verifies a commitment against the implementor commit.
    fn verify_commitment_cb(&self, params: &P, commitment: &C, cb: &Fn(&Self, &P, &C) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, commitment)
    }

    /// Checks a commitment against the implementor commit.
    fn check_commitment_cb(&self, params: &P, commitment: &C, cb: &Fn(&Self, &P, &C) -> Result<()>)
        -> Result<()>
    {
        cb(self, params, commitment)
    }
}
