//! # Numerical
//!
//! `numerical` is the module providing the traits implemented by types that can be added,
//! subtracted or multiplied.

use std::ops::{Add, Sub, Mul};

use base::data::Datable;

/// Trait implemented by types that can be added, subtracted or multiplied.
pub trait Numerical : Sized + Ord + Datable + Add + Sub + Mul {}

impl Numerical for u8 {}

impl Numerical for i8 {}

impl Numerical for u16 {}

impl Numerical for i16 {}

impl Numerical for u32 {}

impl Numerical for i32 {}

impl Numerical for u64 {}

impl Numerical for i64 {}