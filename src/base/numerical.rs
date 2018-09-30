use std::ops::{Add, Sub, Mul};

use base::data::Datable;

pub trait Numerical : Sized + Datable + Add + Sub + Mul {}