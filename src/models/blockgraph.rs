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
use base::Evaluable;
use crypto::{Hashable, Committable, Authenticatable};
use io::{Store, Storable};
use models::Meta;
use models::BlockNode;

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
            P: Datable
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
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>) -> Result<Self>
    {
        params.check()?;

        self.update_size();

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `BlockGraph`.
    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>) -> Result<D>
    {
        params.check()?;

        let mut bg = self.clone();
        bg.id = D::default();

        bg.digest_cb(params, cb)
    }

    /// Verifies the cryptographic digest against the `BlockGraph`'s digest.
    pub fn verify_digest<HP: Datable>(&self,
                                      params: &HP,
                                      cb: &Fn(&Self, &HP, &D) -> Result<bool>)
        -> Result<bool>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut bg = self.clone();
        bg.id = D::default();
        bg.update_size();

        bg.verify_digest_cb(params, &digest, cb)
    }

    /// Checks the cryptographic digest against the `BlockGraph`'s digest.
    pub fn check_digest<HP: Datable>(&self,
                                     params: &HP,
                                     cb: &Fn(&Self, &HP, &D) -> Result<()>)
        -> Result<()>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut bg = self.clone();
        bg.id = D::default();
        bg.update_size();

        bg.check_digest_cb(params, &digest, cb)
    }

    /// Commits cryptographically the `BlockGraph`.
    pub fn commit<CP, C>(&self, params: &CP, cb: &Fn(&Self, &CP) -> Result<C>)
        -> Result<C>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;

        self.commit_cb(params, cb)
    }

    /// Verifies the cryptographic commitment against the `BlockGraph`'s commitment.
    pub fn verify_commitment<CP, C>(&self,
                                    params: &CP,
                                    commitment: &C,
                                    cb: &Fn(&Self, &CP, &C) -> Result<bool>)
        -> Result<bool>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;
        commitment.check()?;

        self.verify_commitment_cb(params, commitment, cb)
    }

    /// Checks the cryptographic commitment against the `BlockGraph`'s commitment.
    pub fn check_commitment<CP, C>(&self,
                                   params: &CP,
                                   commitment: &C,
                                   cb: &Fn(&Self, &CP, &C) -> Result<()>)
        -> Result<()>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;
        commitment.check()?;

        self.check_commitment_cb(params, commitment, cb)
    }

    /// Authenticates cryptographically the `BlockGraph`.
    pub fn authenticate<AP, T>(&self, params: &AP, cb: &Fn(&Self, &AP) -> Result<T>)
        -> Result<T>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;

        self.authenticate_cb(params, cb)
    }

    /// Verifies the cryptographic authentication of the `BlockGraph` against a tag.
    pub fn verify_tag<AP, T>(&self,
                             params: &AP,
                             tag: &T,
                             cb: &Fn(&Self, &AP, &T) -> Result<bool>)
        -> Result<bool>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;
        tag.check()?;

        self.verify_tag_cb(params, tag, cb)
    }

    /// Checks the cryptographic authentication of the `BlockGraph` against a tag.
    pub fn check_tag<AP, T>(&self,
                            params: &AP,
                            tag: &T,
                            cb: &Fn(&Self, &AP, &T) -> Result<()>)
        -> Result<()>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;
        tag.check()?;

        self.check_tag_cb(params, tag, cb)
    }

    /// Evals the `BlockGraph`.
    pub fn eval<EP, R>(&self, params: &EP, cb: &Fn(&Self, &EP) -> Result<R>)
        -> Result<R>
        where   EP: Datable,
                R: Datable
    {
        params.check()?;

        self.eval_cb(params, cb)
    }
}

impl<HP, D, P> Hashable<HP, D> for BlockGraph<D, P>
    where   HP: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{}

impl<CP, C, D, P> Committable<CP, C> for BlockGraph<D, P>
    where   CP: Datable,
            C: Datable + ConstantSize,
            D: Ord + Datable + ConstantSize,
            P: Datable
{}

impl<AP, T, D, P> Authenticatable<AP, T> for BlockGraph<D, P>
    where   AP: Datable,
            T: Datable + ConstantSize,
            D: Ord + Datable + ConstantSize,
            P: Datable
{}

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

impl<D, P> Evaluable for BlockGraph<D, P>
    where   D: Ord + Datable + ConstantSize,
            P: Datable
{}

impl<St, S, D, P, StP, StPC, StRC>
    Storable<St, S, D, BlockGraph<D, P>, StP, StPC, StRC>
    for BlockGraph<D, P>
    where   St: Store<S, D, BlockGraph<D, P>, StP, StPC, StRC>,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            P: Datable + Serializable,
            StP: Datable,
            StPC: Datable + Serializable,
            StRC: Datable + Serializable
{
    fn store_key(&self) -> Result<D> {
        self.id.check()?;

        Ok(self.id.clone())
    }

    fn store_value(&self) -> Result<BlockGraph<D, P>> {
        self.check()?;

        Ok(self.clone())
    }
}