//! # Check
//!
//! `check` is the module providing the trait implemented by types that can be checked.

use base::result::Result;
use base::future::Future;

/// Trait implemented by types that can be checked.
pub trait Checkable
{
    /// Checks the implementor.
    fn check(&self) -> Result<()> {
        Ok(())
    }

    /// Check asynchronously the implementor.
    fn check_async(&self) -> Future<()> {
        Future::<()>::from_result(self.check())
    }
}

impl Checkable for () {}

impl Checkable for bool {}

impl Checkable for u8 {}

impl Checkable for i8 {}

impl Checkable for u16 {}

impl Checkable for i16 {}

impl Checkable for u32 {}

impl Checkable for i32 {}

impl Checkable for u64 {}

impl Checkable for i64 {}

impl Checkable for String {}

impl<T> Checkable for Vec<T>
    where   T: Checkable
{}

impl<T> Checkable for Option<T>
    where   T: Checkable
{}

impl<T> Checkable for Box<T>
    where   T: Checkable
{}