//! # BlockGraph
//!
//! `blockgraph` is the module providing the type used to represent a graph of authenticated `Block`s.
//! Nodes are `Blocknode`s and edges the links between them, which are specified in the `Block`s
//! referenced by the graph `BlockNode`s.
//! An authenticated graph allows to represent different authenticated data structures
//! (linked lists, trees, sets, etc), so it is a natural choice to keep the framework generic.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize};
use base::{Eval, EvalMut};
use base::Meta;
use crypto::Hash;
use io::{Store, Storable};
use model::BlockNode;

/// Type representing a graph of `BlockNodes`. It just expose the graph frontier, from which
/// one can span the entire graph after following the `BlockNode`s' `Block`s `prev_block_id` links.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct BlockGraph<D, P>
    where   D: Ord + Datable + ConstantSize,
            P: Datable
{
    /// BlockGraph id. It is the digest of the same blockgraph, but with a default `D` id.
    pub id: D,
    /// BlockGraph metadata.
    pub meta: Meta,
    /// Blockgraph tip's or frontier's height.
    pub height: u64,
    /// BlockGraph's frontier tip idx, if any.
    pub tip_idx: Option<u64>,
    /// BlockGraph's frontier length.
    pub frontier_len: u64,
    /// BlockGraph's frontier.
    pub frontier: Vec<BlockNode<D>>,
    /// Custom payload.
    pub payload: P,
}

impl<D, P> BlockGraph<D, P>
    where   D: Ord + Datable + ConstantSize,
            P: Datable,
            Self: Serializable
{
    /// Creates a new `BlockGraph`.
    pub fn new() -> Self {
        let mut bg = BlockGraph::default();
        bg.update_size();
        bg
    }

    /// Updates the `BlockGraph` size.
    pub fn update_size(&mut self) {
        let size = self.size();

        self.meta.set_size(size);
    }

    /// Sets the `BlockGraph`'s metadata.
    pub fn meta(mut self, meta: &Meta) -> Result<Self> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `BlockGraph`s frontier and its height and lenght.
    pub fn frontier(mut self, tip_idx: Option<u64>, frontier: &Vec<BlockNode<D>>) -> Result<Self>
    {
        frontier.check()?;

        let mut height = 0;

        for node in frontier.clone() {
            if node.block_height > height {
                height = node.block_height;
            }
        }

        if let Some(idx) = tip_idx {
            if idx > (frontier.len() -1) as u64 {
                return Err(String::from("invalid tip index"));
            }
        }

        self.height = height;
        self.tip_idx = tip_idx;
        self.frontier_len = frontier.len() as u64;
        self.frontier = frontier.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `BlockGraph`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Self> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Finalizes the `BlockGraph`, building its id and returning it's complete form.
    pub fn finalize<H: Hash<D>>(mut self, hasher: &mut H) -> Result<Self> {
        let msg = self.to_bytes()?;
        self.id = hasher.digest(&msg)?;

        self.update_size();

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `BlockGraph`.
    pub fn digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<D> {
        let mut blockgraph = self.clone();
        blockgraph.id = D::default();
        blockgraph.update_size();

        let msg = blockgraph.to_bytes()?;
        hasher.digest(&msg)
    }

    /// Verifies the cryptographic digest against the `BlockGraph`'s digest.
    pub fn verify_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<bool> {
        let digest = self.id.clone();
        digest.check()?;

        let mut blockgraph = self.clone();
        blockgraph.id = D::default();
        blockgraph.update_size();

        let msg = blockgraph.to_bytes()?;
        hasher.verify(&msg, &digest)
    }

    /// Checks the cryptographic digest against the `BlockGraph`'s digest.
    pub fn check_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<()> {
        let digest = self.id.clone();
        digest.check()?;

        let mut blockgraph = self.clone();
        blockgraph.id = D::default();
        blockgraph.update_size();

        let msg = blockgraph.to_bytes()?;
        hasher.check(&msg, &digest)
    }

    /// Evals the `BlockGraph`.
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

    /// Evals mutably the `BlockGraph`.
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

impl<D, P> Sizable for BlockGraph<D, P>
    where   D: Ord + Datable + ConstantSize,
            P: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.height.size() +
            self.tip_idx.size() +
            self.frontier_len.size() +
            self.frontier.size() +
            self.payload.size()
    }
}

impl<D, P> Checkable for BlockGraph<D, P>
    where   D: Ord + Datable + ConstantSize,
            P: Datable
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.get_size() != self.size() {
            return Err(String::from("invalid meta size"));
        }
        
        self.height.check()?;
        self.tip_idx.check()?;
        self.frontier_len.check()?;

        if self.frontier.len() != self.frontier_len as usize {
            return Err(String::from("invalid frontier length"));
        }
        
        for node in self.frontier.clone() {
            node.check()?;
        }

        if let Some(idx) = self.tip_idx {
            if idx > self.frontier_len -1 {
                return Err(String::from("invalid frontier tip idx"));
            }
        }

        self.payload.check()?;

        Ok(())
    }
}

impl<D, P> Serializable for BlockGraph<D, P>
    where   D: Ord + Datable + ConstantSize + Serializable,
            P: Datable + Serializable
{}

impl<D, P> Datable for BlockGraph<D, P>
    where   D: Ord + Datable + ConstantSize,
            P: Datable
{}

pub const BLOCKGRAPH_STORE_PREFIX: u64 = 6;

impl<St, S, D, P>
    Storable<St, S, D, BlockGraph<D, P>>
    for BlockGraph<D, P>
    where   St: Store<S>,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            P: Datable + Serializable
{
    fn store_prefix() -> u64 {
        BLOCKGRAPH_STORE_PREFIX
    }

    fn store_key(&self) -> Result<D> {
        self.id.check()?;

        Ok(self.id.clone())
    }

    fn store_value(&self) -> Result<Self> {
        self.check()?;

        Ok(self.clone())
    }
}