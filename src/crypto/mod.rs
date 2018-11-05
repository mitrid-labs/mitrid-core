//! # Crypto
//!
//! `crypto` is the module providing the traits used to implement cryptographic functionalities.

/// Trait implemented by types that can be cryptographically hashed.
pub mod hash;

/// Trait implemented by types that can be cryptographically signed.
pub mod sign;

/// Trait implemented by types that can be committed in a cryptographic commitment scheme.
pub mod commit;

/// Trait implemented by types that can be cryptographically authenticated.
pub mod authenticate;

/// Trait implemented by types that can be proved and verified cryptographically.
pub mod prove;

pub use self::hash::{Hashable, Hash};
pub use self::sign::{Signable, Sign};
pub use self::commit::{Committable, Commit};
pub use self::authenticate::{Authenticatable, Authenticate};
pub use self::prove::{Provable, Prove};