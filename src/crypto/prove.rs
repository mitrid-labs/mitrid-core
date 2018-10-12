//! # Provable
//!
//! `provable` is the module providing the trait used to implement cryptographic proof-systems.

use base::Result;
use base::Datable;

/// Trait implemented by types that can produce cryptographic proofs and verify them.
pub trait Provable<P, Pr>
    where   P: Datable,
            Pr: Datable,
            Self: 'static + Sized
{
    /// Creates a cryptographic proof by using `Datable` params and a callback.
    fn prove_cb(&self, params: &P, cb: &Fn(&Self, &P) -> Result<Pr>) -> Result<Pr> {
        cb(self, params)
    }

    /// Verifies a cryptographic proof against the implementor.
    fn verify_proof_cb(&self, params: &P, proof: &Pr, cb: &Fn(&Self, &P, &Pr) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, proof)
    }

    /// Checks a cryptographic proof against the implementor.
    fn check_proof_cb(&self, params: &P, proof: &Pr, cb: &Fn(&Self, &P, &Pr) -> Result<()>)
        -> Result<()>
    {
        cb(self, params, proof)
    }
}
