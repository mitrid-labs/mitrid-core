//! # Data
//!
//! `data` is the module providing the trait implemented by types that can be used
//! as generic types in struct fields and function parameters.

use std::fmt::Debug;
use base::check::Checkable;
use base::size::Sizable;

/// Trait implemented by generic types that can be used as fields or parameters.
pub trait Datable : 'static + Sized + Clone + Eq + Debug + Default + Sizable + Checkable {}

impl Datable for () {}

impl Datable for u8 {}

impl Datable for u32 {}

impl Datable for u64 {}

impl Datable for i8 {}

impl Datable for i32 {}

impl Datable for i64 {}

impl Datable for String {}

impl<T> Datable for Vec<T>
    where   T: Datable
{}

impl<T> Datable for Option<T>
    where   T: Datable
{}

impl<T> Datable for Box<T>
    where   T: Datable
{}