//! # Response
//!
//! `response` is the module providing the type representing an application command response.

use base::Result;
use base::Sizable;
use base::Checkable;
use base::Serializable;
use base::Datable;
use base::Meta;

/// Type used to represent an application command response.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Response<A, R>
    where   A: Ord + Datable,
            R: Datable,
{
    /// App address.
    pub address: A,
    /// Response metadata.
    pub meta: Meta,
    /// Response result, if any.
    pub result: Option<R>,
    /// Response error, if any.
    pub error: Option<String>,
}

impl<A, R> Response<A, R>
    where   A: Ord + Datable,
            R: Datable
{
    /// Creates a new none `Response`.
    pub fn new(address: &A, meta: &Meta, result: Option<R>, error: Option<String>) -> Result<Self> {
        address.check()?;
        meta.check()?;
        result.check()?;
        error.check()?;

        if error.is_some() && result.is_some() {
            return Err(format!("invalid result"));
        }

        let mut response = Response {
            address: address.to_owned(),
            meta: meta.to_owned(),
            result: result.to_owned(),
            error: error.to_owned(),
        };

        let size = response.size();

        response.meta.set_size(size);

        Ok(response)
    }
}

impl<A, R> Sizable for Response<A, R>
    where   A: Ord + Datable,
            R: Datable,
{
    fn size(&self) -> u64 {
        self.address.size() +
            self.meta.size() +
            self.result.size()
    }
}

impl<A, R> Checkable for Response<A, R>
    where   A: Ord + Datable,
            R: Datable,
{
    fn check(&self) -> Result<()> {
        self.address.check()?;
        self.meta.check()?;
        self.result.check()?;
        self.error.check()?;

        if self.error.is_some() && self.result.is_some() {
            return Err(format!("invalid result"));
        }

        if self.meta.get_size() != self.size() {
            return Err(format!("invalid size"));
        }

        Ok(())
    }
}

impl<A, R> Serializable for Response<A, R>
    where   A: Ord + Datable  + Serializable,
            R: Datable + Serializable,
{}

impl<A, R> Datable for Response<A, R>
    where   A: Ord + Datable,
            R: Datable,
{}