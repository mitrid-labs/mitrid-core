//! # Response
//!
//! `response` is the module providing the type representing network response messages.

use base::Result;
use base::{Sizable, ConstantSize, VariableSize};
use base::Checkable;
use base::Serializable;
use base::Datable;
use io::store::{Store, Storable};
use io::network::message::Method;
use io::network::message::Resource;
use io::network::message::Message;

/// Type representing a network response message.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Response<S, Ad, NP, D, P>
    where   S: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable,
{
    /// Response inner message
    pub message: Message<S, Ad, NP, D, P>,
}

impl<S, Ad, NP, D, P> Response<S, Ad, NP, D, P>
    where   S: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable,
{
    /// Creates a new `Response`.
    pub fn new(msg: &Message<S, Ad, NP, D, P>) -> Result<Self> {
        msg.check()?;

        let res = Response { message: msg.to_owned() };
        Ok(res)
    }

    /// Returns the `Response` method.
    pub fn method(&self) -> Method {
        self.message.method.clone()
    }

    /// Returns the `Response` resource.
    pub fn resource(&self) -> Resource {
        self.message.resource.clone()
    }

    /// Returns the `Response` payload.
    pub fn payload(&self) -> P {
        self.message.payload.clone()
    }

    /// Returns if the `Response` is an error response.
    pub fn is_error(&self) -> Result<bool> {
        self.message.is_error()
    }
}

impl<S, Ad, NP, D, P> Sizable for Response<S, Ad, NP, D, P>
    where   S: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable,
{
    fn size(&self) -> u64 {
        self.message.size()
    }
}

impl<S, Ad, NP, D, P> Checkable for Response<S, Ad, NP, D, P>
    where   S: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable,
{
    fn check(&self) -> Result<()> {
        self.message.check()
    }
}

impl<S, Ad, NP, D, P> Serializable for Response<S, Ad, NP, D, P>
    where   S: Datable + Serializable,
            Ad: Ord + Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            P: Datable + Serializable,
{}

impl<S, Ad, NP, D, P> Datable for Response<S, Ad, NP, D, P>
    where   S: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable,
{}

impl<St, S, MS, Ad, NP, D, P, StP, StPC, StRC>
    Storable<St, S, D, Response<MS, Ad, NP, D, P>, StP, StPC, StRC>
    for Response<MS, Ad, NP, D, P>
    where   St: Store<S, StP, StPC, StRC>,
            S: Datable + Serializable,
            MS: Datable + Serializable,
            Ad: Ord + Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            P: Datable + Serializable,
            StP: Datable,
            StPC: Datable + Serializable,
            StRC: Datable + Serializable
{
    fn store_key(&self) -> Result<D> {
        self.message.id.check()?;

        Ok(self.message.id.clone())
    }

    fn store_value(&self) -> Result<Self> {
        self.check()?;

        Ok(self.clone())
    }
}