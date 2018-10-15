//! # Authenticate
//!
//! `authenticate` is the module providing the trait used to implement cryptographic authentication.

use base::Result;
use base::ConstantSize;
use base::Datable;

/// Trait used by types that can be cryptographically authenticated.
pub trait Authenticated<P, T>
    where   P: Datable,
            T: Datable + ConstantSize,
            Self: Datable
{
    /// Authenticates cryptographhically the implementor using `Datable` params and a callback.
    /// Returns an authentication tag.
    fn authenticate_cb(&self, params: &P, cb: &Fn(&Self, &P) -> Result<T>) -> Result<T> {
        cb(self, params)
    }

    /// Verifies an authentication tag against the implementor tag.
    fn verify_tag_cb(&self, params: &P, tag: &T, cb: &Fn(&Self, &P, &T) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, tag)
    }

    /// Checks an authentication tag against the implementor tag.
    fn check_tag_cb(&self, params: &P, tag: &T, cb: &Fn(&Self, &P, &T) -> Result<()>)
        -> Result<()>
    {
        cb(self, params, tag)
    }
}
