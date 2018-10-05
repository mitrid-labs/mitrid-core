//! # BlockNode
//!
//! `blocknode` is the module providing the type used to represent a node in the `BlockGraph`.
//! A `BlockNode` references a `Block`.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use crypto::Hashable;
use models::Meta;

/// Type used to represent a node in the `BlockGraph` and that references a `Block`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct BlockNode<D>
    where   D: Datable + FixedSize
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
    where   D: Datable + FixedSize
{
    /// Creates a new `BlockNode`.
    pub fn new() -> BlockNode<D> {
        BlockNode::default()
    }

    /// Sets the `BlockNode`'s metadata.
    pub fn meta(mut self, meta: &Meta) -> Result<BlockNode<D>> {
        meta.check()?;
        self.meta = meta.clone();

        Ok(self)
    }

    /// Sets the `BlockNode`'s block data (block_id, block_height).
    pub fn block_data(mut self, block_id: &D, block_height: u64)
        -> Result<BlockNode<D>>
    {
        block_id.check()?;
        block_id.check_size()?;

        self.block_id = block_id.clone();
        self.block_height = block_height;

        Ok(self)
    }

    /// Finalizes the `BlockNode`, building its id and returning it's complete form.
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<BlockNode<D>>
    {
        params.check()?;

        self.meta.size = self.size();

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `BlockNode`.
    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<D>
    {
        params.check()?;

        self.digest_cb(params, cb)
    }

    /// Verifies the cryptographic digest against the `BlockNode`'s digest.
    pub fn verify_digest<HP: Datable>(&self,
                                      params: &HP,
                                      digest: &D,
                                      cb: &Fn(&Self, &HP, &D) -> Result<bool>)
        -> Result<bool>
    {
        params.check()?;
        digest.check()?;

        self.verify_digest_cb(params, digest, cb)
    }

    /// Checks the cryptographic digest against the `BlockNode`'s digest.
    pub fn check_digest<HP: Datable>(&self,
                                     params: &HP,
                                     digest: &D,
                                     cb: &Fn(&Self, &HP, &D) -> Result<bool>)
        -> Result<()>
    {
        params.check()?;
        digest.check()?;

        self.check_digest_cb(params, digest, cb)
    }
}

impl<P, D> Hashable<P, D> for BlockNode<D>
    where   P: Datable,
            D: Datable + FixedSize
{}

impl<D> Sizable for BlockNode<D>
    where   D: Datable + FixedSize
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.block_id.size() +
            self.block_height.size()
    }
}

impl<D> Checkable for BlockNode<D>
    where   D: Datable + FixedSize
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.size != self.size() {
            return Err(String::from("invalid meta size"));
        }
        
        self.block_id.check()?;
        self.block_id.check_size()?;

        Ok(())
    }
}

impl<D> Serializable for BlockNode<D>
    where   D: Datable + FixedSize + Serializable
{}

impl<D> Datable for BlockNode<D>
    where   D: Datable + FixedSize
{}