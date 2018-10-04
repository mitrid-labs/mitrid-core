//! # Crypto
//!
//! `crypto` is the module providing the traits used to implement cryptographic functionalities.

/// Trait implemented by types that can be generated randomly.
pub mod rand;

/// Trait implemented by types that can be cryptographically hashed.
pub mod hash;

/// Traits implemented by cryptographic key generators.
pub mod keys;

/// Trait implemented by types that can be cryptographically signed.
pub mod sign;

/// Trait implemented by types that can be committed in a cryptographic commitment scheme.
pub mod commit;

/// Trait implemented by types that can be cryptographically authenticated.
pub mod authenticate;

/// Trait implemented by types that can be proved and verified cryptographically.
pub mod prove;

pub use self::rand::Random;
pub use self::hash::Hashable;
pub use self::keys::{Key, KeyPair};
pub use self::sign::Signable;
pub use self::commit::Committable;
pub use self::authenticate::Authenticated;
pub use self::prove::Provable;