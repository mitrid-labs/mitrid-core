//! # Timestamp
//!
//! `timestamp` is the module providing the `Timestamp` type and methods.

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::ops::{Add, AddAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Sub, Div, Rem};

use base::Result;
use base::Sizable;
use base::Checkable;
use base::Datable;
use base::Serializable;
use utils::timestamp::TimestampDiff;

/// Type representing a numerical timestamp. The 0 is set to the Unix Epoch Time, 01-01-1970.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct Timestamp(u64);

impl Timestamp {
    /// Creates a new timestamp.
    pub fn new(tmstmp: u64) -> Timestamp {
        Timestamp::from_u64(tmstmp)
    }

    /// Creates a new timestamp from an `u64`.
    pub fn from_u64(tmsmp: u64) -> Timestamp {
        Timestamp(tmsmp)
    }

    /// Creates a new timestamp from a `Duration` starting at the Unix Epoch Time (01-01-1970).
    pub fn from_unix_epoch_duration(dur: Duration) -> Timestamp {
        let tmstmp = dur.as_secs() * 1000 + (dur.subsec_millis() as u64);
        Timestamp(tmstmp)
    }

    /// Creates a new timestamp from milliseconds.
    pub fn from_millis(millis: u64) -> Timestamp {
        Timestamp::from_u64(millis)
    }

    /// Creates a new timestamp from seconds.
    pub fn from_secs(secs: u64) -> Timestamp {
        Timestamp::from_u64(secs * 1000)
    }

    /// Converts the `Timestamp` to a `Duration`.
    pub fn as_duration(&self) -> Duration {
        let tmstmp = self.as_u64();
        let secs = tmstmp / 1000;
        let millis = tmstmp % 1000;
        let secs_dur = Duration::from_secs(secs);
        let millis_dur = Duration::from_millis(millis);

        secs_dur + millis_dur
    }

    /// Converts the `Timestamp` to a `TimestampDiff`.
    pub fn as_diff(&self) -> TimestampDiff {
        TimestampDiff::from_u64(self.0)
    }

    /// Converts the `Timestamp` to `u64`.
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    /// Converts the `Timestamp` to milliseconds.
    pub fn as_millis(&self) -> u64 {
        self.as_u64()
    }

    /// Converts the `Timestamp` to seconds.
    pub fn as_secs(&self) -> u64 {
        self.as_u64() / 1000
    }

    /// Returns the current time timestamp.
    pub fn now() -> Result<Timestamp> {
        let dur = SystemTime::now().duration_since(UNIX_EPOCH)
                    .map_err(|e| format!("{}", e))?;

        let tmstmp = Timestamp::from_unix_epoch_duration(dur);
        Ok(tmstmp)
    }
}

impl Default for Timestamp {
    fn default() -> Timestamp {
        Timestamp::now().unwrap()
    }
}

impl Add<TimestampDiff> for Timestamp {
    type Output = Timestamp;

    fn add(self, other: TimestampDiff) -> Timestamp {
        let tmsmp = ((self.as_u64() as i64) + other.as_i64()) as u64;
        Timestamp(tmsmp)
    }
}

impl<'a> Add<TimestampDiff> for &'a Timestamp {
    type Output = Timestamp;

    fn add(self, other: TimestampDiff) -> Timestamp {
        let tmsmp = ((self.as_u64() as i64) + other.as_i64()) as u64;
        Timestamp(tmsmp)
    }
}

impl<'a> Add<&'a TimestampDiff> for Timestamp {
    type Output = Timestamp;

    fn add(self, other: &'a TimestampDiff) -> Timestamp {
        let tmsmp = ((self.as_u64() as i64) + other.as_i64()) as u64;
        Timestamp(tmsmp)
    }
}

impl AddAssign<TimestampDiff> for Timestamp {
    fn add_assign(&mut self, other: TimestampDiff) {
        let tmsmp = ((self.as_u64() as i64) + other.as_i64()) as u64;
        self.0 = tmsmp;
    }
}

impl<'a> AddAssign<&'a TimestampDiff> for Timestamp {
    fn add_assign(&mut self, other: &'a TimestampDiff) {
        let tmsmp = ((self.as_u64() as i64) + other.as_i64()) as u64;
        self.0 = tmsmp;
    }
}

impl<'a, 'b> AddAssign<&'b TimestampDiff> for &'a mut Timestamp {
    fn add_assign(&mut self, other: &'b TimestampDiff) {
        let tmsmp = ((self.as_u64() as i64) + other.as_i64()) as u64;
        self.0 = tmsmp;
    }
}

impl Sub<Timestamp> for Timestamp {
    type Output = TimestampDiff;

    fn sub(self, other: Timestamp) -> TimestampDiff {
        let diff = (self.as_u64() as i64) - (other.as_u64() as i64);
        TimestampDiff::new(diff)
    }
}

impl<'a> Sub<Timestamp> for &'a Timestamp {
    type Output = TimestampDiff;

    fn sub(self, other: Timestamp) -> TimestampDiff {
        let diff = (self.as_u64() as i64) - (other.as_u64() as i64);
        TimestampDiff::new(diff)
    }
}

impl<'a> Sub<&'a Timestamp> for Timestamp {
    type Output = TimestampDiff;

    fn sub(self, other: &'a Timestamp) -> TimestampDiff {
        let diff = (self.as_u64() as i64) - (other.as_u64() as i64);
        TimestampDiff::new(diff)
    }
}

impl Mul<u64> for Timestamp {
    type Output = Timestamp;

    fn mul(self, multiplier: u64) -> Timestamp {
        Timestamp(self.0 * multiplier)
    }
}

impl<'a> Mul<u64> for &'a Timestamp {
    type Output = Timestamp;

    fn mul(self, multiplier: u64) -> Timestamp {
        Timestamp(self.0 * multiplier)
    }
}

impl<'a> Mul<&'a u64> for Timestamp {
    type Output = Timestamp;

    fn mul(self, multiplier: &'a u64) -> Timestamp {
        Timestamp(self.0 * multiplier)
    }
}

impl MulAssign<u64> for Timestamp {
    fn mul_assign(&mut self, multiplier: u64) {
        self.0 *= multiplier
    }
}

impl<'a> MulAssign<&'a u64> for Timestamp {
    fn mul_assign(&mut self, multiplier: &'a u64) {
        self.0 *= multiplier
    }
}

impl<'a, 'b> MulAssign<&'b u64> for &'a mut Timestamp {
    fn mul_assign(&mut self, multiplier: &'b u64) {
        self.0 *= multiplier
    }
}

impl Div for Timestamp {
    type Output = u64;

    fn div(self, other: Timestamp) -> u64 {
        self.0 / other.0
    }
}

impl<'a> Div for &'a Timestamp {
    type Output = u64;

    fn div(self, other: &'a Timestamp) -> u64 {
        self.0 / other.0
    }
}

impl<'a> Div<&'a Timestamp> for Timestamp {
    type Output = u64;

    fn div(self, other: &'a Timestamp) -> u64 {
        self.0 / other.0
    }
}

impl Rem for Timestamp {
    type Output = u64;

    fn rem(self, other: Timestamp) -> u64 {
        self.0 % other.0
    }
}

impl<'a> Rem for &'a Timestamp {
    type Output = u64;

    fn rem(self, other: &'a Timestamp) -> u64 {
        self.0 % other.0
    }
}

impl<'a> Rem<&'a Timestamp> for Timestamp {
    type Output = u64;

    fn rem(self, other: &'a Timestamp) -> u64 {
        self.0 % other.0
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
