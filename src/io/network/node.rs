//! # Node
//!
//! `node` is the module providing the type used to represent a node in the distributed ledger network.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, VariableSize};
use base::Evaluable;
use utils::Meta;
use io::{Store, Storable};

/// Type representing a node in the distributed ledger network.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Node<A, P>
    where   A: Ord + Datable + VariableSize,
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
    where   A: Ord + Datable + VariableSize,
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
    where   A: Ord + Datable + VariableSize,
            P: Datable
{
    fn size(&self) -> u64 {
        self.meta.size() +
            self.address.size() +
            self.payload.size()
    }
}

impl<A, P> Checkable for Node<A, P>
    where   A: Ord + Datable + VariableSize,
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
    where   A: Ord + Datable + VariableSize + Serializable,
            P: Datable + Serializable
{}

impl<A, P> Datable for Node<A, P>
    where   A: Ord + Datable + VariableSize,
            P: Datable
{}

impl<A, P> Evaluable for Node<A, P>
    where   A: Ord + Datable + VariableSize,
            P: Datable
{}

pub const NODE_STORE_PREFIX: u64 = 8;

impl<St, S, A, P, StP, StPC, StRC>
    Storable<St, S, A, Node<A, P>, StP, StPC, StRC>
    for Node<A, P>
    where   St: Store<S, StP, StPC, StRC>,
            S: Datable + Serializable,
            A: Ord + Datable + VariableSize + Serializable,
            P: Datable + Serializable,
            StP: Datable,
            StPC: Datable + Serializable,
            StRC: Datable + Serializable
{
    fn store_prefix() -> u64 {
        NODE_STORE_PREFIX
    }

    fn store_key(&self) -> Result<A> {
        self.address.check()?;

        Ok(self.address.clone())
    }

    fn store_value(&self) -> Result<Self> {
        self.check()?;

        Ok(self.clone())
    }
}