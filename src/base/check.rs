use base::result::Result;
use base::future::Future;

pub trait Checkable
{
    fn check(&self) -> Result<()> {
        Ok(())
    }

    fn check_async(&self) -> Future<()> {
        Future::<()>::from_result(self.check())
    }
}

impl Checkable for () {}

impl Checkable for u8 {}

impl Checkable for u32 {}

impl Checkable for u64 {}

impl Checkable for i8 {}

impl Checkable for i32 {}

impl Checkable for i64 {}

impl Checkable for f32 {}

impl Checkable for f64 {}

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