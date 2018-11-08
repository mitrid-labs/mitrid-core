//! # Node
//!
//! `node` is the module providing the type used to represent a node in the distributed ledger network.

use std::mem;

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, VariableSize};
use base::{Eval, EvalMut};
use base::Meta;
use io::{Store, Storable};

/// Code of the `Node` type.
pub const NODE_CODE: u64 = 8;

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
    pub fn eval<Ev, EP, ER>(&self, params: &EP, evaluator: &Ev)
        -> Result<ER>
        where   Ev: Eval<Self, EP, ER>,
                EP: Datable,
                ER: Datable
    {
        self.check()?;
        params.check()?;

        evaluator.eval(self, params)
    }

    /// Evals mutably the `Node`.
    pub fn eval_mut<EvM, EP, ER>(&mut self, params: &EP, evaluator: &mut EvM)
        -> Result<ER>
        where   EvM: EvalMut<Self, EP, ER>,
                EP: Datable,
                ER: Datable
    {
        self.check()?;
        params.check()?;

        let result = evaluator.eval_mut(self, params)?;
        self.update_size();

        self.check()?;

        Ok(result)
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

impl<St, S, A, P>
    Storable<St, S, A, Node<A, P>>
    for Node<A, P>
    where   St: Store<S>,
            S: Datable + Serializable,
            A: Ord + Datable + VariableSize + Serializable,
            P: Datable + Serializable
{
    fn store_prefix() -> Vec<u8> {
        let mut prefix = Vec::new();

        let _prefix: [u8; 8] = unsafe { mem::transmute(NODE_CODE) };
        prefix.extend_from_slice(&_prefix[..]);

        prefix
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