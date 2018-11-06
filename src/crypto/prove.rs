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
    fn check(&mut self, msg: &[u8], proof: &P) -> Result<()>;
}