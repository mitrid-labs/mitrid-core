//! # TimestampDiff
//!
//! `timestamp_diff` is the module providing the `TimestampDiff` type and methods. This type
//! represents the difference between two `Timestamp`s.

use std::time::Duration;
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::Div;
use std::ops::Rem;

use base::Sizable;
use base::Checkable;
use base::Datable;
use base::Serializable;

/// Type representing the difference between two `Timestamp`s.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct TimestampDiff(i64);

impl TimestampDiff {
    /// Creates a new `TimestampDiff`.
    pub fn new(tmdiff: i64) -> TimestampDiff {
        TimestampDiff::from_i64(tmdiff)
    }

    /// Creates a new `TimestampDiff` from `i64`.
    pub fn from_i64(tmdiff: i64) -> TimestampDiff {
        TimestampDiff(tmdiff)
    }

    /// Creates a new `TimestampDiff` from `u64`.
    pub fn from_u64(tmdiff: u64) -> TimestampDiff {
        TimestampDiff(tmdiff as i64)
    }

    /// Creates a new `TimestampDiff` from a `Duration`.
    pub fn from_duration(dur: Duration) -> TimestampDiff {
        let _diff = dur.as_secs() * 1000 + (dur.subsec_millis() as u64);
        TimestampDiff(_diff as i64)
    }

    /// Creates a new `TimestampDiff` from milliseconds.
    pub fn from_millis(millis: u64) -> TimestampDiff {
        TimestampDiff::from_u64(millis)
    }

    /// Creates a new `TimestampDiff` from seconds.
    pub fn from_secs(secs: u64) -> TimestampDiff {
        TimestampDiff::from_u64(secs * 1000)
    }

    /// Converts the `TimestampDiff` to `i64`.
    pub fn as_i64(&self) -> i64 {
        self.0
    }

    /// Converts the `TimestampDiff` to its absolute representation.
    pub fn abs(&self) -> u64 {
        self.0.abs() as u64
    }

    /// Converts the `TimestampDiff` to `u64`.
    pub fn as_u64(&self) -> u64 {
        self.abs()
    }
 
    /// Converts the `TimestampDiff` to a `Duration`.
    pub fn as_duration(&self) -> Duration {
        let abs = self.abs();
        let secs = abs / 1000;
        let millis = abs % 1000;
        let secs_dur = Duration::from_secs(secs);
        let millis_dur = Duration::from_millis(millis);

        secs_dur + millis_dur
    }

    /// Converts the `TimestampDiff` to milliseconds.
    pub fn as_millis(&self) -> u64 {
        self.as_u64()
    }

    /// Converts the `TimestampDiff` to seconds.
    pub fn as_secs(&self) -> u64 {
        self.as_u64() / 1000
    }

    /// Returns the sign of the `TimestampDiff`.
    pub fn sign(&self) -> i64 {
        self.as_i64() / (self.abs() as i64)
    }
}

impl Add<TimestampDiff> for TimestampDiff {
    type Output = TimestampDiff;

    fn add(self, other: TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 + other.0)
    }
}

impl<'a> Add<TimestampDiff> for &'a TimestampDiff {
    type Output = TimestampDiff;

    fn add(self, other: TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 + other.0)
    }
}

impl<'a> Add<&'a TimestampDiff> for TimestampDiff {
    type Output = TimestampDiff;

    fn add(self, other: &'a TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 + other.0)
    }
}

impl<'a, 'b> Add<&'b TimestampDiff> for &'a TimestampDiff {
    type Output = TimestampDiff;

    fn add(self, other: &'b TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 + other.0)
    }
}

impl AddAssign for TimestampDiff {
    fn add_assign(&mut self, other: TimestampDiff) {
        self.0 += other.0
    }
}

impl<'a> AddAssign<&'a TimestampDiff> for TimestampDiff {
    fn add_assign(&mut self, other: &'a TimestampDiff) {
        self.0 += other.0
    }
}

impl<'a> AddAssign<TimestampDiff> for &'a mut TimestampDiff {
    fn add_assign(&mut self, other: TimestampDiff) {
        self.0 += other.0
    }
}

impl<'a, 'b> AddAssign<&'b TimestampDiff> for &'a mut TimestampDiff {
    fn add_assign(&mut self, other: &'b TimestampDiff) {
        self.0 += other.0
    }
}

impl Sub<TimestampDiff> for TimestampDiff {
    type Output = TimestampDiff;

    fn sub(self, other: TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 - other.0)
    }
}

impl<'a> Sub<TimestampDiff> for &'a TimestampDiff {
    type Output = TimestampDiff;

    fn sub(self, other: TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 - other.0)
    }
}

impl<'a> Sub<&'a TimestampDiff> for TimestampDiff {
    type Output = TimestampDiff;

    fn sub(self, other: &'a TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 - other.0)
    }
}

impl<'a, 'b> Sub<&'b TimestampDiff> for &'a TimestampDiff {
    type Output = TimestampDiff;

    fn sub(self, other: &'b TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 - other.0)
    }
}

impl SubAssign for TimestampDiff {
    fn sub_assign(&mut self, other: TimestampDiff) {
        self.0 -= other.0
    }
}

impl<'a> SubAssign<&'a TimestampDiff> for TimestampDiff {
    fn sub_assign(&mut self, other: &'a TimestampDiff) {
        self.0 -= other.0
    }
}

impl<'a> SubAssign<TimestampDiff> for &'a mut TimestampDiff {
    fn sub_assign(&mut self, other: TimestampDiff) {
        self.0 -= other.0
    }
}

impl<'a, 'b> SubAssign<&'b TimestampDiff> for &'a mut TimestampDiff {
    fn sub_assign(&mut self, other: &'b TimestampDiff) {
        self.0 -= other.0
    }
}

impl Mul<i64> for TimestampDiff {
    type Output = TimestampDiff;

    fn mul(self, other: i64) -> TimestampDiff {
        TimestampDiff(self.0 * other)
    }
}

impl<'a> Mul<i64> for &'a TimestampDiff {
    type Output = TimestampDiff;

    fn mul(self, other: i64) -> TimestampDiff {
        TimestampDiff(self.0 * other)
    }
}

impl<'a> Mul<&'a i64> for TimestampDiff {
    type Output = TimestampDiff;

    fn mul(self, other: &'a i64) -> TimestampDiff {
        TimestampDiff(self.0 * other)
    }
}

impl<'a, 'b> Mul<&'b i64> for &'a TimestampDiff {
    type Output = TimestampDiff;

    fn mul(self, other: &'b i64) -> TimestampDiff {
        TimestampDiff(self.0 * other)
    }
}

impl MulAssign<i64> for TimestampDiff {
    fn mul_assign(&mut self, other: i64) {
        self.0 *= other
    }
}

impl<'a> MulAssign<&'a i64> for TimestampDiff {
    fn mul_assign(&mut self, other: &'a i64) {
        self.0 *= other
    }
}

impl<'a> MulAssign<i64> for &'a mut TimestampDiff {
    fn mul_assign(&mut self, other: i64) {
        self.0 *= other
    }
}

impl<'a, 'b> MulAssign<&'b i64> for &'a mut TimestampDiff {
    fn mul_assign(&mut self, other: &'b i64) {
        self.0 *= other
    }
}

impl Div<TimestampDiff> for TimestampDiff {
    type Output = i64;

    fn div(self, other: TimestampDiff) -> i64 {
        self.0 / other.0
    }
}

impl<'a> Div<TimestampDiff> for &'a TimestampDiff {
    type Output = i64;

    fn div(self, other: TimestampDiff) -> i64 {
        self.0 / other.0
    }
}

impl<'a> Div<&'a TimestampDiff> for TimestampDiff {
    type Output = i64;

    fn div(self, other: &'a TimestampDiff) -> i64 {
        self.0 / other.0
    }
}

impl<'a, 'b> Div<&'b TimestampDiff> for &'a TimestampDiff {
    type Output = i64;

    fn div(self, other: &'b TimestampDiff) -> i64 {
        self.0 / other.0
    }
}

impl Rem<TimestampDiff> for TimestampDiff {
    type Output = i64;

    fn rem(self, other: TimestampDiff) -> i64 {
        self.0 % other.0
    }
}

impl<'a> Rem<&'a TimestampDiff> for TimestampDiff {
    type Output = i64;

    fn rem(self, other: &'a TimestampDiff) -> i64 {
        self.0 % other.0
    }
}

impl<'a> Rem<TimestampDiff> for &'a TimestampDiff {
    type Output = i64;

    fn rem(self, other: TimestampDiff) -> i64 {
        self.0 % other.0
    }
}

impl<'a, 'b> Rem<&'b TimestampDiff> for &'a TimestampDiff {
    type Output = i64;

    fn rem(self, other: &'b TimestampDiff) -> i64 {
        self.0 % other.0
    }
}

impl Sizable for TimestampDiff {
    fn size(&self) -> u64 {
        self.0.size()
    }
}

impl Checkable for TimestampDiff {}

impl Datable for TimestampDiff {}

impl Serializable for TimestampDiff {}