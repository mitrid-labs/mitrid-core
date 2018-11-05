//! # Base
//!
//! `base` is the module providing the basic traits used throughoug `mitrid-core`. They are
//! designed to allow the library user to customize the framework easily with custom
//! data structures.

/// The Result type used across the library.
pub mod result;

/// Trait to be implemented by types that can be checked.
pub mod check;

/// Trait to be implemented by types that can be fields or parameters. 
pub mod data;

/// Trait to be implemented by types that can be serialized.
pub mod serialize;

/// Trait to be implemented by types that can be sized.
pub mod size;

/// Trait to be implemented by types that can be added, subtracted, multiplied.
pub mod numerical;

/// Trait to be implemented by types that can be evaluated (computed).
pub mod eval;

/// Type used to define the distributed ledger stage.
pub mod stage;

/// Type used to convey the distributed ledger metadata.
pub mod meta;

pub use self::result::Result;
pub use self::check::Checkable;
pub use self::data::Datable;
pub use self::serialize::Serializable;
pub use self::numerical::Numerical;
pub use self::size::{Sizable, VariableSize, ConstantSize};
pub use self::eval::{Eval, EvalMut};
pub use self::stage::Stage;
pub use self::meta::Meta;