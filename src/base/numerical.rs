//! # Numerical
//!
//! `numerical` is the module providing the traits implemented by types that can be added,
//! subtracted or multiplied.

use std::ops::{Add, Sub, Mul};

use base::data::Datable;

/// Trait implemented by types that can be added, subtracted or multiplied.
pub trait Numerical : Sized + Datable + Add + Sub + Mul {}