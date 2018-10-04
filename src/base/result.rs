//! # Result
//!
//! `result` is the module providing the `Result` type used throughout the library.

use std::result::Result as StdResult;

/// Alias type to a `std::result::Result` with `String` error.
pub type Result<T> = StdResult<T, String>;