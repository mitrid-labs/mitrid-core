//! # OnError
//!
//! `on_error` is the module providing the type representing the behaviour of clients on an error response.

use base::size::Sizable;
use base::check::Checkable;
use base::serialize::Serializable;
use base::data::Datable;

/// Type used to represent the client behaviour on error.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[allow(unused_attributes)]
pub enum OnError {
    /// On error, ignore.
    #[repr(u8)]
    Ignore,
    /// On error, fail.
    #[repr(u8)]
    Fail,
    /// On error, retry a given times and ignore afterwards.
    RetryAndIgnore(u64),
    /// On error, retry a given times and fail afterwards.
    RetryAndFail(u64),
}

impl OnError {
    /// Ignores on error.
    pub fn new_ignore() -> OnError {
        OnError::Ignore
    }

    /// Fails on error.
    pub fn new_error() -> OnError {
        OnError::Fail
    }

    /// Retry a given times and ignore afterwards.
    pub fn new_retry_and_ignore(n: u64) -> OnError {
        OnError::RetryAndIgnore(n)
    }

    /// Retry a given times and fail afterwards.
    pub fn new_retry_and_fail(n: u64) -> OnError {
        OnError::RetryAndFail(n)
    }
}

impl Default for OnError {
    fn default() -> Self {
        OnError::Ignore
    }
}

impl Sizable for OnError {
    fn size(&self) -> u64 {
        1
    }
}

impl Checkable for OnError {}

impl Serializable for OnError {}

impl Datable for OnError {}