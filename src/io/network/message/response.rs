//! # Response
//!
//! `response` is the module providing the type representing network response messages.

use base::Result;
use base::Numerical;
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
pub struct Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
{
    /// Response inner message
    pub message: Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>,
}

impl<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
{
    /// Creates a new `Response`.
    pub fn new(msg: &Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>) -> Result<Self> {
        msg.check()?;

        let res = Response { message: msg.to_owned() };
        Ok(res)
    }

    /// Returns the `Response` method.
    pub fn method(&self) -> Method {
        self.message.method.clone()
    }

    /// Returns the `Response` resource.
    pub fn resource(&self) -> Resource<RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> {
        self.message.resource.clone()
    }

    /// Returns if the `Response` is an error response.
    pub fn is_error(&self) -> Result<bool> {
        self.message.is_error()
    }
}

impl<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Sizable
    for Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
{
    fn size(&self) -> u64 {
        self.message.size()
    }
}

impl<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Checkable
    for Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
{
    fn check(&self) -> Result<()> {
        self.message.check()
    }
}

impl<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Serializable
    for Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable + Serializable,
            RS: Datable + Serializable,
            Ad: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable,
            Pr: Datable + Serializable,
            Am: Numerical + Serializable,
            IP: Datable + Serializable,
            OP: Datable + Serializable,
            TP: Datable + Serializable,
            BP: Datable + Serializable,
            BGP: Datable + Serializable,
            C: Datable + Serializable
{}

impl<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Datable
    for Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
{}

impl<St, S, MS, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C, StP, StPC, StRC>
    Storable<St, S, D, Response<MS, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>, StP, StPC, StRC>
    for Response<MS, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   St: Store<S, D, Response<MS, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>, StP, StPC, StRC>,
            S: Datable + Serializable,
            MS: Datable + Serializable,
            RS: Datable + Serializable,
            Ad: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable,
            Pr: Datable + Serializable,
            Am: Numerical + Serializable,
            IP: Datable + Serializable,
            OP: Datable + Serializable,
            TP: Datable + Serializable,
            BP: Datable + Serializable,
            BGP: Datable + Serializable,
            C: Datable + Serializable,
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