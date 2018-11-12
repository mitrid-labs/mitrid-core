//! # Data
//!
//! `data` is the module providing the trait implemented by types that can be used
//! as generic types in struct fields and function parameters.

use std::any::Any;
use std::fmt::Debug;
use base::check::Checkable;
use base::size::Sizable;

/// Trait implemented by generic types that can be used as fields or parameters.
pub trait Datable : 'static + Send + Sync + Sized + Clone + Eq + Debug + Default + Sizable + Checkable {}

impl Datable for () {}

impl Datable for bool {}

impl Datable for u8 {}

impl Datable for i8 {}

impl Datable for u16 {}

impl Datable for i16 {}

impl Datable for u32 {}

impl Datable for i32 {}

impl Datable for u64 {}

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

impl<A, B> Datable for (A, B)
    where   A: Datable,
            B: Datable
{}

impl<A, B, C> Datable for (A, B, C)
    where   A: Datable,
            B: Datable,
            C: Datable
{}

impl<A, B, C, D> Datable for (A, B, C, D)
    where   A: Datable,
            B: Datable,
            C: Datable,
            D: Datable
{}

impl<A, B, C, D, E> Datable for (A, B, C, D, E)
    where   A: Datable,
            B: Datable,
            C: Datable,
            D: Datable,
            E: Datable
{}

impl<A, B, C, D, E, F> Datable for (A, B, C, D, E, F)
    where   A: Datable,
            B: Datable,
            C: Datable,
            D: Datable,
            E: Datable,
            F: Datable
{}