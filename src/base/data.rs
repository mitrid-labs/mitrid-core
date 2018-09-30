use std::fmt::Debug;
use base::check::Checkable;
use base::size::Sizable;

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