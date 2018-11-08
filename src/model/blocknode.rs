//! # BlockNode
//!
//! `blocknode` is the module providing the type used to represent a node in the `BlockNode`.
//! A `BlockNode` references a `Block`.

use std::mem;

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize};
use base::{Eval, EvalMut};
use base::Meta;
use crypto::Hash;
use io::{Store, Storable};

/// Code of the `BlockNode` type.
pub const BLOCKNODE_CODE: u64 = 4;

/// Type used to represent a node in the `BlockNode` and that references a `Block`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct BlockNode<D>
    where   D: Ord + Datable + ConstantSize
{
    /// BlockNode id. It is the digest of the same coin, but with a default `D` id.
    pub id: D,
    /// BlockNode metadata.
    pub meta: Meta,
    /// BBlockNode's block's id.
    pub block_id: D,
    /// BlockNode's Block's height.
    pub block_height: u64
}

impl<D> BlockNode<D>
    where   D: Ord + Datable + ConstantSize,
            Self: Serializable
{
    /// Creates a new `BlockNode`.
    pub fn new() -> Self {
        let mut bn = BlockNode::default();
        bn.update_size();
        bn
    }

    /// Updates the `BlockNode` size.
    pub fn update_size(&mut self) {
        let size = self.size();

        self.meta.set_size(size);
    }

    /// Sets the `BlockNode`'s metadata.
    pub fn meta(mut self, meta: &Meta) -> Result<Self> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `BlockNode`'s block data (block_id, block_height).
    pub fn block_data(mut self, block_id: &D, block_height: u64) -> Result<Self>
    {
        block_id.check()?;
        block_id.check_size()?;

        self.block_id = block_id.clone();
        self.block_height = block_height;

        self.update_size();

        Ok(self)
    }

    /// Finalizes the `BlockNode`, building its id and returning it's complete form.
    pub fn finalize<H: Hash<D>>(mut self, hasher: &mut H) -> Result<Self> {
        let msg = self.to_bytes()?;
        self.id = hasher.digest(&msg)?;

        self.update_size();

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `BlockNode`.
    pub fn digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<D> {
        let mut blocknode = self.clone();
        blocknode.id = D::default();
        blocknode.update_size();

        let msg = blocknode.to_bytes()?;
        hasher.digest(&msg)
    }

    /// Verifies the cryptographic digest against the `BlockNode`'s digest.
    pub fn verify_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<bool> {
        let digest = self.id.clone();
        digest.check()?;

        let mut blocknode = self.clone();
        blocknode.id = D::default();
        blocknode.update_size();

        let msg = blocknode.to_bytes()?;
        hasher.verify(&msg, &digest)
    }

    /// Checks the cryptographic digest against the `BlockNode`'s digest.
    pub fn check_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<()> {
        let digest = self.id.clone();
        digest.check()?;

        let mut blocknode = self.clone();
        blocknode.id = D::default();
        blocknode.update_size();

        let msg = blocknode.to_bytes()?;
        hasher.check(&msg, &digest)
    }

    /// Evals the `BlockNode`.
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

    /// Evals mutably the `BlockNode`.
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

impl<D> Sizable for BlockNode<D>
    where   D: Ord + Datable + ConstantSize
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.block_id.size() +
            self.block_height.size()
    }
}

impl<D> Checkable for BlockNode<D>
    where   D: Ord + Datable + ConstantSize
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.get_size() != self.size() {
            return Err(String::from("invalid meta size"));
        }
        
        self.block_id.check()?;
        self.block_id.check_size()?;

        Ok(())
    }
}

impl<D> Serializable for BlockNode<D>
    where   D: Ord + Datable + ConstantSize + Serializable
{}

impl<D> Datable for BlockNode<D>
    where   D: Ord + Datable + ConstantSize
{}

impl<St, S, D>
    Storable<St, S, D, BlockNode<D>>
    for BlockNode<D>
    where   St: Store<S>,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable
{
    fn store_prefix() -> Vec<u8> {
        let mut prefix = Vec::new();

        let _prefix: [u8; 8] = unsafe { mem::transmute(BLOCKNODE_CODE) };
        prefix.extend_from_slice(&_prefix[..]);

        prefix
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