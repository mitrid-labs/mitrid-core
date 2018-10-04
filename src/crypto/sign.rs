//! # Sign
//!
//! `sign` is the module providing the trait used to produce and verify cryptographic signatures.

use base::Result;
use base::Datable;
use base::FixedSize;

/// Trait implemented by types that can produce and verify cryptographic signatures.
pub trait Signable<P, Sk, Pk, Sig>
    where   P: Datable,
            Sk: Datable + FixedSize,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            Self: 'static + Sized
{
    /// Signs cryptographically the implementor using `Datable` params, a secret key and a callback.
    /// Returns a cryptographic signature.
    fn sign_cb(&self, params: &P, sk: &Sk, cb: &Fn(&Self, &P, &Sk) -> Result<Sig>) -> Result<Sig> {
        cb(self, params, sk)
    }

    /// Verifies a cryptographic signature against the implementor using a public key and a callback.
    fn verify_signature_cb(&self,
                           params: &P,
                           sig: &Sig,
                           pk: &Pk,
                           cb: &Fn(&Self, &P, &Sig, &Pk) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, sig, pk)
    }

    /// Checks a cryptographic signature against the implementor using a public key and a callback.
    fn check_signature_cb(&self,
                          params: &P,
                          sig: &Sig,
                          pk: &Pk,
                          cb: &Fn(&Self, &P, &Sig, &Pk) -> Result<bool>)
        -> Result<()>
    {
        if !Self::verify_signature_cb(self, params, sig, pk, cb)? {
            return Err(String::from("invalid signature"));
        }

        Ok(())
    }
}