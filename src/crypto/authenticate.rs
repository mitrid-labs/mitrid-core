//! # Authenticate
//!
//! `authenticate` is the module providing the trait used to implement cryptographic authentication.

use base::Result;
use base::ConstantSize;
use base::Datable;

/// Trait used by types that implements cryptographic authentication.
pub trait Authenticate<K, T>
    where   K: Datable + ConstantSize,
            T: Datable + ConstantSize
{
    /// Generates an authentication key.
    fn generate_key(&mut self) -> Result<K>;

    /// Authenticates cryptographhically the message using an authentication key
    /// and returning its authentication tag.
    fn authenticate(&mut self, msg: &[u8], key: &K) -> Result<T>;

    /// Verifies an authentication tag against a message using an authentication key.
    fn verify(&mut self, msg: &[u8], key: &K, tag: &T) -> Result<bool>;

    /// Checks an authentication tag against a message using an authentication key.
    fn check(&mut self, msg: &[u8], key: &K, tag: &T) -> Result<()> {
        key.check()?;
        key.check_size()?;
        tag.check()?;
        tag.check_size()?;

        if !self.verify(msg, key, tag)? {
            return Err(format!("invalid tag"));
        }

        Ok(())
    }
}