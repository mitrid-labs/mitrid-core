use std::time::Duration;
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Rem, RemAssign};

use base::Sizable;
use base::Checkable;
use base::Datable;
use base::Serializable;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct TimestampDiff(u64);

impl TimestampDiff {
    pub fn new(tmstmp: u64) -> TimestampDiff {
        TimestampDiff::from_u64(tmstmp)
    }

    pub fn from_u64(tmstmp: u64) -> TimestampDiff {
        TimestampDiff(tmstmp)
    }

    pub fn from_duration(dur: Duration) -> TimestampDiff {
        let _diff = dur.as_secs() * 1000 + (dur.subsec_millis() as u64);
        TimestampDiff(_diff)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn as_duration(&self) -> Duration {
        let secs = self.0 / 1000;
        let millis = self.0 % 1000;
        let secs_dur = Duration::from_secs(secs);
        let millis_dur = Duration::from_millis(millis);

        secs_dur + millis_dur
    }
}

impl Add for TimestampDiff {
    type Output = TimestampDiff;

    fn add(self, other: TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 + other.0)
    }
}

impl<'a> Add for &'a TimestampDiff {
    type Output = TimestampDiff;

    fn add(self, other: &'a TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 + other.0)
    }
}

impl<'a> Add<&'a TimestampDiff> for TimestampDiff {
    type Output = TimestampDiff;

    fn add(self, other: &'a TimestampDiff) -> TimestampDiff {
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

impl<'a, 'b> AddAssign<&'b TimestampDiff> for &'a mut TimestampDiff {
    fn add_assign(&mut self, other: &'b TimestampDiff) {
        self.0 += other.0
    }
}

impl Sub for TimestampDiff {
    type Output = TimestampDiff;

    fn sub(self, other: TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 - other.0)
    }
}

impl<'a> Sub for &'a TimestampDiff {
    type Output = TimestampDiff;

    fn sub(self, other: &'a TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 - other.0)
    }
}

impl<'a> Sub<&'a TimestampDiff> for TimestampDiff {
    type Output = TimestampDiff;

    fn sub(self, other: &'a TimestampDiff) -> TimestampDiff {
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

impl<'a, 'b> SubAssign<&'b TimestampDiff> for &'a mut TimestampDiff {
    fn sub_assign(&mut self, other: &'b TimestampDiff) {
        self.0 -= other.0
    }
}

impl Mul for TimestampDiff {
    type Output = TimestampDiff;

    fn mul(self, other: TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 * other.0)
    }
}

impl<'a> Mul for &'a TimestampDiff {
    type Output = TimestampDiff;

    fn mul(self, other: &'a TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 * other.0)
    }
}

impl<'a> Mul<&'a TimestampDiff> for TimestampDiff {
    type Output = TimestampDiff;

    fn mul(self, other: &'a TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 * other.0)
    }
}

impl MulAssign for TimestampDiff {
    fn mul_assign(&mut self, other: TimestampDiff) {
        self.0 *= other.0
    }
}

impl<'a> MulAssign<&'a TimestampDiff> for TimestampDiff {
    fn mul_assign(&mut self, other: &'a TimestampDiff) {
        self.0 *= other.0
    }
}

impl<'a, 'b> MulAssign<&'b TimestampDiff> for &'a mut TimestampDiff {
    fn mul_assign(&mut self, other: &'b TimestampDiff) {
        self.0 *= other.0
    }
}

impl Div for TimestampDiff {
    type Output = TimestampDiff;

    fn div(self, other: TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 / other.0)
    }
}

impl<'a> Div for &'a TimestampDiff {
    type Output = TimestampDiff;

    fn div(self, other: &'a TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 / other.0)
    }
}

impl<'a> Div<&'a TimestampDiff> for TimestampDiff {
    type Output = TimestampDiff;

    fn div(self, other: &'a TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 / other.0)
    }
}

impl DivAssign for TimestampDiff {
    fn div_assign(&mut self, other: TimestampDiff) {
        self.0 /= other.0
    }
}

impl<'a> DivAssign<&'a TimestampDiff> for TimestampDiff {
    fn div_assign(&mut self, other: &'a TimestampDiff) {
        self.0 /= other.0
    }
}

impl<'a, 'b> DivAssign<&'b TimestampDiff> for &'a mut TimestampDiff {
    fn div_assign(&mut self, other: &'b TimestampDiff) {
        self.0 /= other.0
    }
}

impl Rem for TimestampDiff {
    type Output = TimestampDiff;

    fn rem(self, other: TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 % other.0)
    }
}

impl<'a> Rem for &'a TimestampDiff {
    type Output = TimestampDiff;

    fn rem(self, other: &'a TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 % other.0)
    }
}

impl<'a> Rem<&'a TimestampDiff> for TimestampDiff {
    type Output = TimestampDiff;

    fn rem(self, other: &'a TimestampDiff) -> TimestampDiff {
        TimestampDiff(self.0 % other.0)
    }
}

impl RemAssign for TimestampDiff {
    fn rem_assign(&mut self, other: TimestampDiff) {
        self.0 %= other.0
    }
}

impl<'a> RemAssign<&'a TimestampDiff> for TimestampDiff {
    fn rem_assign(&mut self, other: &'a TimestampDiff) {
        self.0 %= other.0
    }
}

impl<'a, 'b> RemAssign<&'b TimestampDiff> for &'a mut TimestampDiff {
    fn rem_assign(&mut self, other: &'b TimestampDiff) {
        self.0 %= other.0
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