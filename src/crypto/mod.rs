//! # Crypto
//!
//! `crypto` is the module providing the traits used to implement cryptographic functionalities.

/// Trait implemented by types that implement cryptographic hashing.
pub mod hash;

/// Trait implemented by types that implement cryptographic signing.
pub mod sign;

/// Trait implemented by types that implement cryptographic commitment.
pub mod commit;

/// Trait implemented by types that implement cryptographic authenticated.
pub mod authenticate;

/// Trait implemented by types that implement cryptographic proving.
pub mod prove;

pub use self::hash::Hash;
pub use self::sign::Sign;
pub use self::commit::Commit;
pub use self::authenticate::Authenticate;
pub use self::prove::Prove;