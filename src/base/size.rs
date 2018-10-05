//! # Size
//!
//! `size` is the module providing the traits implemented by types that can be sized.

use base::result::Result;

/// Trait implemented by types that have a size.
pub trait Sizable {
    /// Returns the size of the implementor.
    fn size(&self) -> u64;
}

impl Sizable for () {
    fn size(&self) -> u64 {
        0
    }
}

impl Sizable for bool {
    fn size(&self) -> u64 {
        1
    }
}

impl Sizable for u8 {
    fn size(&self) -> u64 {
        1
    }
}

impl Sizable for i8 {
    fn size(&self) -> u64 {
        1
    }
}

impl Sizable for u16 {
    fn size(&self) -> u64 {
        2
    }
}

impl Sizable for i16 {
    fn size(&self) -> u64 {
        2
    }
}

impl Sizable for u32 {
    fn size(&self) -> u64 {
        4
    }
}

impl Sizable for i32 {
    fn size(&self) -> u64 {
        4
    }
}

impl Sizable for u64 {
    fn size(&self) -> u64 {
        8
    }
}

impl Sizable for i64 {
    fn size(&self) -> u64 {
        8
    }
}

impl Sizable for String {
    fn size(&self) -> u64 {
        (self.len() * 2) as u64
    }
}

impl<T> Sizable for Vec<T>
    where   T: Sizable
{
    fn size(&self) -> u64 {
        let mut res = 0u64;

        for el in self.clone() {
            res += el.size();
        }

        res
    }
}

impl<T> Sizable for Option<T>
    where   T: Sizable
{
    fn size(&self) -> u64 {
        if let Some(internal) = self.clone() {
            internal.size()
        } else {
            0
        }
    }
}

impl<T> Sizable for Box<T>
    where   T: Sizable
{
    #[allow(unconditional_recursion)] // risky
    fn size(&self) -> u64 {
        self.size()
    }
}

/// Trait implemented by types that have a variable size.
pub trait VariableSize:
    where   Self: Sizable
{
    /// Returns the minimum size of the implementor.
    fn min_size() -> u64 {
        0
    }

    /// Returns the maximum size of the implementor, if any.
    fn max_size() -> Option<u64> {
        None
    }

    /// Check the size of the implementor.
    fn check_size(&self) -> Result<()> {
        if self.size() < Self::min_size() {
            return Err(String::from("size under the minimum size"))
        }

        if let Some(max_size) = Self::max_size() {
            if self.size() > max_size {
                return Err(String::from("size over the maximum size"))
            }
        }

        Ok(())
    }
}

/// Trait implemented by types that have a fixed size.
pub trait FixedSize:
    where   Self: Sizable
{
    /// Returns the size required by the implementor.
    fn required_size() -> u64;

    /// Check the size of the implementor.
    fn check_size(&self) -> Result<()> {
        if self.size() != Self::required_size() {
            return Err(String::from("size different from the required size"))
        }

        Ok(())
    }
}