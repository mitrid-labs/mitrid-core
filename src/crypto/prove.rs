//! # Provable
//!
//! `provable` is the module providing the trait used to implement cryptographic proof-systems.

use base::Result;
use base::Datable;

/// Trait used by types that implements cryptographic proof.
pub trait Prove<P>
    where   P: Datable,
            Self: 'static + Sized
{
    /// Proves cryptographically a message, returning a proof.
    fn prove(&mut self, msg: &[u8]) -> Result<P>;

    /// Verifies a proof against a message.
    fn verify(&mut self, msg: &[u8], proof: &P) -> Result<bool>;

    /// Checks a proof against a message.
    fn check(&mut self, msg: &[u8], proof: &P) -> Result<()> {
        if !self.verify(msg, proof)? {
            return Err(format!("invalid proof"));
        }

        Ok(())
    }
}

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
