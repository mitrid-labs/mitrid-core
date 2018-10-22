//! # Node
//!
//! `node` is the module providing the type used to represent a node in the distributed ledger network.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, VariableSize};
use base::Evaluable;
use models::Meta;

/// Type representing a node in the distributed ledger network.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Node<A, P>
    where   A: Datable + VariableSize,
            P: Datable
{
    /// Node metadata.
    pub meta: Meta,
    /// Node address.
    pub address: A,
    /// Custom payload.
    pub payload: P,
}

impl<A, P> Node<A, P>
    where   A: Datable + VariableSize,
            P: Datable
{
    /// Creates a new `Node`.
    pub fn new(meta: &Meta, address: &A, payload: &P) -> Result<Node<A, P>> {
        meta.check()?;
        address.check()?;
        address.check_size()?;
        payload.check()?;

        let mut node = Node {
            meta: meta.to_owned(),
            address: address.to_owned(),
            payload: payload.to_owned(),
        };

        node.update_size();

        Ok(node)
    }

    /// Updates the `Node` size.
    pub fn update_size(&mut self) {
        let size = self.size();

        self.meta.set_size(size);
    }

    /// Evals the `Node`.
    pub fn eval<EP, R>(&self, params: &EP, cb: &Fn(&Self, &EP) -> Result<R>)
        -> Result<R>
        where   EP: Datable,
                R: Datable
    {
        params.check()?;

        self.eval_cb(params, cb)
    }
}

impl<A, P> Sizable for Node<A, P>
    where   A: Datable + VariableSize,
            P: Datable
{
    fn size(&self) -> u64 {
        self.meta.size() +
            self.address.size() +
            self.payload.size()
    }
}

impl<A, P> Checkable for Node<A, P>
    where   A: Datable + VariableSize,
            P: Datable
{
    fn check(&self) -> Result<()> {
        self.meta.check()?;

        if self.meta.get_size() != self.size() {
            return Err("invalid size".into());
        }
        
        self.address.check()?;
        self.payload.check()?;

        Ok(())
    }
}

impl<A, P> Serializable for Node<A, P>
    where   A: Datable + VariableSize + Serializable,
            P: Datable + Serializable
{}

impl<A, P> Datable for Node<A, P>
    where   A: Datable + VariableSize,
            P: Datable
{}

impl<A, P> Evaluable for Node<A, P>
    where   A: Datable + VariableSize,
            P: Datable
{}