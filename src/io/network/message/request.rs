//! # Request
//!
//! `request` is the module providing the type representing network request messages.

use base::Result;
use base::Numerical;
use base::{Sizable, ConstantSize, VariableSize};
use base::Checkable;
use base::Serializable;
use base::Datable;
use io::store::Storable;
use io::network::message::Resource;
use io::network::message::Message;

/// Type representing a network request message.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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
    /// Request inner message
    pub message: Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>,
}

impl<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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
    /// Creates a new `Request`.
    pub fn new(msg: &Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>) -> Result<Self> {
        msg.check()?;

        match msg.resource {
            Resource::Error(_) => {
                Err(String::from("invalid resource"))
            },
            _ => {
                let req = Request { message: msg.to_owned() };
                Ok(req)
            },
        }
    }
}

impl<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Sizable
    for Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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
    for Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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
    for Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable + Serializable,
            RS: Datable + Serializable,
            Ad: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Datable + ConstantSize + Serializable,
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
    for Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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

impl<S, MS, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    Storable<S, D, Request<MS, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>
    for Request<MS, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable + Serializable,
            MS: Datable + Serializable,
            RS: Datable + Serializable,
            Ad: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Datable + ConstantSize + Serializable,
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