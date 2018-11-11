//! # Response
//!
//! `response` is the module providing the type representing network response messages.

use std::mem;

use base::Result;
use base::{Sizable, ConstantSize};
use base::Checkable;
use base::Serializable;
use base::Datable;
use io::store::{Store, Storable};
use io::network::message::Message;

/// Code of the `Response` type.
pub const RESPONSE_CODE: u64 = 10;

/// Type representing a network response message.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Response<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{
    /// Response inner message
    pub message: Message<S, D, P>,
}

impl<S, D, P> Response<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{
    /// Creates a new `Response`.
    pub fn new(msg: &Message<S, D, P>) -> Result<Self> {
        msg.check()?;

        let res = Response { message: msg.to_owned() };
        Ok(res)
    }
}

impl<S, D, P> Sizable for Response<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{
    fn size(&self) -> u64 {
        self.message.size()
    }
}

impl<S, D, P> Checkable for Response<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{
    fn check(&self) -> Result<()> {
        self.message.check()
    }
}

impl<S, D, P> Serializable for Response<S, D, P>
    where   S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            P: Datable + Serializable
{}

impl<S, D, P> Datable for Response<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{}

impl<St, S, MS, D, P>
    Storable<St, S, D, Response<MS, D, P>>
    for Response<MS, D, P>
    where   St: Store<S>,
            S: Datable + Serializable,
            MS: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            P: Datable + Serializable
{
    fn store_prefix() -> Vec<u8> {
        let mut prefix = Vec::new();

        let _prefix: [u8; 8] = unsafe { mem::transmute(RESPONSE_CODE) };
        prefix.extend_from_slice(&_prefix[..]);

        prefix
    }

    fn store_key(&self) -> Result<D> {
        self.message.id.check()?;

        Ok(self.message.id.clone())
    }

    fn store_value(&self) -> Result<Self> {
        self.check()?;

        Ok(self.clone())
    }
}