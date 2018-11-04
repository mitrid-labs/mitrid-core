//! # Request
//!
//! `request` is the module providing the type representing a Mitrid application command request.

use base::Result;
use base::{Sizable, VariableSize};
use base::Checkable;
use base::Serializable;
use base::Datable;
use utils::Meta;

/// Type used to represent an application command request.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Request<A, P>
    where   A: Ord + Datable + VariableSize,
            P: Datable,
{
    /// App address.
    pub address: A,
    /// Request metadata.
    pub meta: Meta,
    /// Request params.
    pub params: P,
}

impl<A, P> Request<A, P>
    where   A: Ord + Datable + VariableSize,
            P: Datable,
{
    /// Creates a new none `Request`.
    pub fn new(address: &A, meta: &Meta, params: &P) -> Result<Self> {
        address.check()?;
        address.check_size()?;
        meta.check()?;
        params.check()?;

        let mut request = Request {
            address: address.to_owned(),
            meta: meta.to_owned(),
            params: params.to_owned(),
        };

        let size = request.size();

        request.meta.set_size(size);

        Ok(request)
    }
}

impl<A, P> Sizable for Request<A, P>
    where   A: Ord + Datable + VariableSize,
            P: Datable,
{
    fn size(&self) -> u64 {
        self.address.size() +
            self.meta.size() +
            self.params.size()
    }
}

impl<A, P> Checkable for Request<A, P>
    where   A: Ord + Datable + VariableSize,
            P: Datable,
{
    fn check(&self) -> Result<()> {
        self.address.check()?;
        self.address.check_size()?;
        self.meta.check()?;
        self.params.check()?;

        if self.meta.get_size() != self.size() {
            return Err(format!("invalid size"));
        }

        Ok(())
    }
}

impl<A, P> Serializable for Request<A, P>
    where   A: Ord + Datable + VariableSize + Serializable,
            P: Datable + Serializable,
{}

impl<A, P> Datable for Request<A, P>
    where   A: Ord + Datable + VariableSize,
            P: Datable,
{}