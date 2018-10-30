//! # Check
//!
//! `check` is the module providing the trait implemented by types that can be checked.

use base::result::Result;

/// Trait implemented by types that can be checked.
pub trait Checkable
{
    /// Checks the implementor.
    fn check(&self) -> Result<()> {
        Ok(())
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
{
    fn check(&self) -> Result<()> {
        for ref v in self {
            v.check()?;
        }

        Ok(())
    }
}

impl<T> Checkable for Option<T>
    where   T: Checkable
{
    fn check(&self) -> Result<()> {
        if let Some(t) = self {
            t.check()?;
        }

        Ok(())
    }
}

impl<T> Checkable for Box<T>
    where   T: Checkable
{
    #[allow(unconditional_recursion)] // NB
    fn check(&self) -> Result<()> {
        (*self).check()
    }
}

impl<A, B> Checkable for (A, B)
    where   A: Checkable,
            B: Checkable
{
    fn check(&self) -> Result<()> {
        self.0.check()?;
        self.1.check()
    }
}

impl<A, B, C> Checkable for (A, B, C)
    where   A: Checkable,
            B: Checkable,
            C: Checkable
{
    fn check(&self) -> Result<()> {
        self.0.check()?;
        self.1.check()?;
        self.2.check()
    }
}

impl<A, B, C, D> Checkable for (A, B, C, D)
    where   A: Checkable,
            B: Checkable,
            C: Checkable,
            D: Checkable
{
    fn check(&self) -> Result<()> {
        self.0.check()?;
        self.1.check()?;
        self.2.check()?;
        self.3.check()
    }
}

impl<A, B, C, D, E> Checkable for (A, B, C, D, E)
    where   A: Checkable,
            B: Checkable,
            C: Checkable,
            D: Checkable,
            E: Checkable
{
    fn check(&self) -> Result<()> {
        self.0.check()?;
        self.1.check()?;
        self.2.check()?;
        self.3.check()?;
        self.4.check()
    }
}

impl<A, B, C, D, E, F> Checkable for (A, B, C, D, E, F)
    where   A: Checkable,
            B: Checkable,
            C: Checkable,
            D: Checkable,
            E: Checkable,
            F: Checkable
{
    fn check(&self) -> Result<()> {
        self.0.check()?;
        self.1.check()?;
        self.2.check()?;
        self.3.check()?;
        self.4.check()?;
        self.5.check()
    }
}