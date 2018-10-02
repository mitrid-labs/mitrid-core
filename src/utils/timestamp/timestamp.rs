use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Rem, RemAssign};

use base::Result;
use base::Sizable;
use base::Checkable;
use base::Datable;
use base::Serializable;
use utils::timestamp::TimestampDiff;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct Timestamp(TimestampDiff);

impl Timestamp {
    pub fn new() -> Result<Timestamp> {
        Timestamp::now()
    }

    pub fn now() -> Result<Timestamp> {
        let dur = SystemTime::now().duration_since(UNIX_EPOCH)
                    .map_err(|e| format!("{}", e))?;

        let tmstmp = Timestamp::from_unix_epoch_duration(dur);
        Ok(tmstmp)
    }

    pub fn from_u64(tmstmp: u64) -> Timestamp {
        Timestamp(TimestampDiff::new(tmstmp))
    }

    pub fn from_unix_epoch_diff(diff: TimestampDiff) -> Timestamp {
        Timestamp(diff)
    }

    pub fn from_unix_epoch_duration(dur: Duration) -> Timestamp {
        let _diff = dur.as_secs() * 1000 + (dur.subsec_millis() as u64);
        let diff = TimestampDiff::from_u64(_diff);
        Timestamp(diff)
    }

    pub fn as_u64(&self) -> u64 {
        self.0.as_u64()
    }

    pub fn as_duration(&self) -> Duration {
        let tmstmp = self.as_u64();
        let secs = tmstmp / 1000;
        let millis = tmstmp % 1000;
        let secs_dur = Duration::from_secs(secs);
        let millis_dur = Duration::from_millis(millis);

        secs_dur + millis_dur
    }

    pub fn as_diff(&self) -> TimestampDiff {
        self.0
    }
}

impl Default for Timestamp {
    fn default() -> Timestamp {
        Timestamp::now().unwrap()
    }
}

impl Add for Timestamp {
    type Output = Timestamp;

    fn add(self, other: Timestamp) -> Timestamp {
        Timestamp(self.0 + other.0)
    }
}

impl<'a> Add for &'a Timestamp {
    type Output = Timestamp;

    fn add(self, other: &'a Timestamp) -> Timestamp {
        Timestamp(self.0 + other.0)
    }
}

impl<'a> Add<&'a Timestamp> for Timestamp {
    type Output = Timestamp;

    fn add(self, other: &'a Timestamp) -> Timestamp {
        Timestamp(self.0 + other.0)
    }
}

impl AddAssign for Timestamp {
    fn add_assign(&mut self, other: Timestamp) {
        self.0 += other.0
    }
}

impl<'a> AddAssign<&'a Timestamp> for Timestamp {
    fn add_assign(&mut self, other: &'a Timestamp) {
        self.0 += other.0
    }
}

impl<'a, 'b> AddAssign<&'b Timestamp> for &'a mut Timestamp {
    fn add_assign(&mut self, other: &'b Timestamp) {
        self.0 += other.0
    }
}

impl Sub for Timestamp {
    type Output = Timestamp;

    fn sub(self, other: Timestamp) -> Timestamp {
        Timestamp(self.0 - other.0)
    }
}

impl<'a> Sub for &'a Timestamp {
    type Output = Timestamp;

    fn sub(self, other: &'a Timestamp) -> Timestamp {
        Timestamp(self.0 - other.0)
    }
}

impl<'a> Sub<&'a Timestamp> for Timestamp {
    type Output = Timestamp;

    fn sub(self, other: &'a Timestamp) -> Timestamp {
        Timestamp(self.0 - other.0)
    }
}

impl SubAssign for Timestamp {
    fn sub_assign(&mut self, other: Timestamp) {
        self.0 -= other.0
    }
}

impl<'a> SubAssign<&'a Timestamp> for Timestamp {
    fn sub_assign(&mut self, other: &'a Timestamp) {
        self.0 -= other.0
    }
}

impl<'a, 'b> SubAssign<&'b Timestamp> for &'a mut Timestamp {
    fn sub_assign(&mut self, other: &'b Timestamp) {
        self.0 -= other.0
    }
}

impl Mul for Timestamp {
    type Output = Timestamp;

    fn mul(self, other: Timestamp) -> Timestamp {
        Timestamp(self.0 * other.0)
    }
}

impl<'a> Mul for &'a Timestamp {
    type Output = Timestamp;

    fn mul(self, other: &'a Timestamp) -> Timestamp {
        Timestamp(self.0 * other.0)
    }
}

impl<'a> Mul<&'a Timestamp> for Timestamp {
    type Output = Timestamp;

    fn mul(self, other: &'a Timestamp) -> Timestamp {
        Timestamp(self.0 * other.0)
    }
}

impl MulAssign for Timestamp {
    fn mul_assign(&mut self, other: Timestamp) {
        self.0 *= other.0
    }
}

impl<'a> MulAssign<&'a Timestamp> for Timestamp {
    fn mul_assign(&mut self, other: &'a Timestamp) {
        self.0 *= other.0
    }
}

impl<'a, 'b> MulAssign<&'b Timestamp> for &'a mut Timestamp {
    fn mul_assign(&mut self, other: &'b Timestamp) {
        self.0 *= other.0
    }
}

impl Div for Timestamp {
    type Output = Timestamp;

    fn div(self, other: Timestamp) -> Timestamp {
        Timestamp(self.0 / other.0)
    }
}

impl<'a> Div for &'a Timestamp {
    type Output = Timestamp;

    fn div(self, other: &'a Timestamp) -> Timestamp {
        Timestamp(self.0 / other.0)
    }
}

impl<'a> Div<&'a Timestamp> for Timestamp {
    type Output = Timestamp;

    fn div(self, other: &'a Timestamp) -> Timestamp {
        Timestamp(self.0 / other.0)
    }
}

impl DivAssign for Timestamp {
    fn div_assign(&mut self, other: Timestamp) {
        self.0 /= other.0
    }
}

impl<'a> DivAssign<&'a Timestamp> for Timestamp {
    fn div_assign(&mut self, other: &'a Timestamp) {
        self.0 /= other.0
    }
}

impl<'a, 'b> DivAssign<&'b Timestamp> for &'a mut Timestamp {
    fn div_assign(&mut self, other: &'b Timestamp) {
        self.0 /= other.0
    }
}

impl Rem for Timestamp {
    type Output = Timestamp;

    fn rem(self, other: Timestamp) -> Timestamp {
        Timestamp(self.0 % other.0)
    }
}

impl<'a> Rem for &'a Timestamp {
    type Output = Timestamp;

    fn rem(self, other: &'a Timestamp) -> Timestamp {
        Timestamp(self.0 % other.0)
    }
}

impl<'a> Rem<&'a Timestamp> for Timestamp {
    type Output = Timestamp;

    fn rem(self, other: &'a Timestamp) -> Timestamp {
        Timestamp(self.0 % other.0)
    }
}

impl RemAssign for Timestamp {
    fn rem_assign(&mut self, other: Timestamp) {
        self.0 %= other.0
    }
}

impl<'a> RemAssign<&'a Timestamp> for Timestamp {
    fn rem_assign(&mut self, other: &'a Timestamp) {
        self.0 %= other.0
    }
}

impl<'a, 'b> RemAssign<&'b Timestamp> for &'a mut Timestamp {
    fn rem_assign(&mut self, other: &'b Timestamp) {
        self.0 %= other.0
    }
}

impl Sizable for Timestamp {
    fn size(&self) -> u64 {
        self.0.size()
    }
}

impl Checkable for Timestamp {
    fn check(&self) -> Result<()> {
        self.0.check()
    }
}

impl Datable for Timestamp {}

impl Serializable for Timestamp {}
