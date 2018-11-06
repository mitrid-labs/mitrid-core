//! # Sign
//!
//! `sign` is the module providing the trait used to produce and verify cryptographic signatures.

use base::Result;
use base::Datable;
use base::ConstantSize;

/// Trait implemented by types that can produce and verify cryptographic signatures.
pub trait Sign<Seed, Pk, Sk, Sig>
    where   Seed: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Self: 'static + Sized
{
    /// Generates the sign keys.
    fn generate_keys(&mut self, seed: Option<Seed>) -> Result<(Pk, Sk)>;

    /// Signs cryptographically a message using a secret key and returning a cryptographic signature.
    fn sign(&mut self, msg: &[u8], sk: &Sk) -> Result<Sig>;

    /// Verifies a cryptographic signature against a message using a public key.
    fn verify(&mut self, msg: &[u8], pk: &Pk, sig: &Sig) -> Result<bool>;

    /// Checks a cryptographic signature against a message using a public key.
    fn check(&mut self, msg: &[u8], pk: &Pk, sig: &Sig) -> Result<()>;
}