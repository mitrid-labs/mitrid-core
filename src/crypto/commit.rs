//! # Commit
//!
//! `commit` is the module providing the trait used to implement a cryptographic commitment scheme.

use base::Result;
use base::FixedSize;
use base::Datable;

/// Trait used by types that can be cryptographically committed.
pub trait Committable<P, C>
    where   P: Datable,
            C: Datable + FixedSize,
            Self: 'static + Sized
{
    /// Commits cryptographically the implementor using `Datable` params and a callback.
    /// Returns a commitment.
    fn commit_cb(&self, params: &P, cb: &Fn(&Self, &P) -> Result<C>) -> Result<C> {
        cb(self, params)
    }

    /// Verifies a commitment against the implementor commit.
    fn verify_commit_cb(&self, params: &P, commit: &C, cb: &Fn(&Self, &P, &C) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, commit)
    }

    /// Checks a commitment against the implementor commit.
    fn check_commit_cb(&self, params: &P, commit: &C, cb: &Fn(&Self, &P, &C) -> Result<bool>)
        -> Result<()>
    {
        if !Self::verify_commit_cb(self, params, commit, cb)? {
            return Err(String::from("invalid commit"));
        }

        Ok(())
    }
}