//! # Request
//!
//! `request` is the module providing the type representing network request messages.

use std::mem;

use base::Result;
use base::{Sizable, ConstantSize};
use base::Checkable;
use base::Serializable;
use base::Datable;
use io::store::{Store, Storable};
use io::network::message::Resource;
use io::network::message::Message;

/// Code of the `Request` type.
pub const REQUEST_CODE: u64 = 9;

/// Type representing a network request message.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Request<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{
    /// Request inner message
    pub message: Message<S, D, P>,
}

impl<S, D, P> Request<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{
    /// Creates a new `Request`.
    pub fn new(msg: &Message<S, D, P>) -> Result<Self> {
        msg.check()?;

        match msg.resource {
            Resource::Error => {
                Err(String::from("invalid resource"))
            },
            _ => {
                let req = Request { message: msg.to_owned() };
                Ok(req)
            },
        }
    }
}

impl<S, D, P> Sizable for Request<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{
    fn size(&self) -> u64 {
        self.message.size()
    }
}

impl<S, D, P> Checkable for Request<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{
    fn check(&self) -> Result<()> {
        self.message.check()?;

        match self.message.resource {
            Resource::Error => {
                Err(String::from("invalid resource"))
            },
            _ => Ok(()),
        }
    }
}

impl<S, D, P> Serializable for Request<S, D, P>
    where   S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            P: Datable + Serializable
{}

impl<S, D, P> Datable for Request<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{}

impl<St, S, MS, D, P>
    Storable<St, S, D, Request<MS, D, P>>
    for Request<MS, D, P>
    where   St: Store<S>,
            S: Datable + Serializable,
            MS: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            P: Datable + Serializable
{
    fn store_prefix() -> Vec<u8> {
        let mut prefix = Vec::new();

        let _prefix: [u8; 8] = unsafe { mem::transmute(REQUEST_CODE) };
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