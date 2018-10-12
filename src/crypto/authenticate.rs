//! # Authenticate
//!
//! `authenticate` is the module providing the trait used to implement cryptographic authentication.

use base::Result;
use base::FixedSize;
use base::Datable;

/// Trait used by types that can be cryptographically authenticated.
pub trait Authenticated<P, T>
    where   P: Datable,
            T: Datable + FixedSize,
            Self: Datable
{
    /// Authenticates cryptographhically the implementor using `Datable` params and a callback.
    /// Returns an authentication token.
    fn authenticate_cb(&self, params: &P, cb: &Fn(&Self, &P) -> Result<T>) -> Result<T> {
        cb(self, params)
    }

    /// Verifies an authentication token against the implementor token.
    fn verify_token_cb(&self, params: &P, token: &T, cb: &Fn(&Self, &P, &T) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, token)
    }

    /// Checks an authentication token against the implementor token.
    fn check_token_cb(&self, params: &P, token: &T, cb: &Fn(&Self, &P, &T) -> Result<()>)
        -> Result<()>
    {
        cb(self, params, token)
    }
}
