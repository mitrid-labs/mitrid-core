use base::result::Result;

pub trait Sizable {
    fn size(&self) -> u64;
}

pub trait VariableSize:
    where   Self: Sizable
{
    fn min_size() -> u64 {
        0
    }

    fn max_size() -> Option<u64> {
        None
    }

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

pub trait FixedSize:
    where   Self: Sizable
{
    fn required_size() -> u64;

    fn check_size(&self) -> Result<()> {
        if self.size() != Self::required_size() {
            return Err(String::from("size different from the required size"))
        }

        Ok(())
    }
}

impl Sizable for () {
    fn size(&self) -> u64 {
        0
    }
}

impl Sizable for u8 {
    fn size(&self) -> u64 {
        1
    }
}

impl Sizable for u32 {
    fn size(&self) -> u64 {
        4
    }
}

impl Sizable for u64 {
    fn size(&self) -> u64 {
        8
    }
}

impl Sizable for i8 {
    fn size(&self) -> u64 {
        1
    }
}

impl Sizable for i32 {
    fn size(&self) -> u64 {
        4
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