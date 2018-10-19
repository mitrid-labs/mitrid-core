//! # BlockNode
//!
//! `blocknode` is the module providing the type used to represent a node in the `BlockNode`.
//! A `BlockNode` references a `Block`.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize, VariableSize};
use crypto::{Hashable, Committable, Authenticatable};
use io::Storable;
use io::Networkable;
use models::Meta;

/// Type used to represent a node in the `BlockNode` and that references a `Block`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct BlockNode<D>
    where   D: Datable + ConstantSize
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
    where   D: Datable + ConstantSize
{
    /// Creates a new `BlockNode`.
    pub fn new() -> BlockNode<D> {
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
    pub fn meta(mut self, meta: &Meta) -> Result<BlockNode<D>> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

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

        self.update_size();

        Ok(self)
    }

    /// Finalizes the `BlockNode`, building its id and returning it's complete form.
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<BlockNode<D>>
    {
        params.check()?;

        self.update_size();

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `BlockNode`.
    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<D>
    {
        params.check()?;

        let mut bn = self.clone();
        bn.id = D::default();

        bn.digest_cb(params, cb)
    }

    /// Verifies the cryptographic digest against the `BlockNode`'s digest.
    pub fn verify_digest<HP: Datable>(&self,
                                      params: &HP,
                                      cb: &Fn(&Self, &HP, &D) -> Result<bool>)
        -> Result<bool>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut bn = self.clone();
        bn.id = D::default();
        bn.update_size();

        bn.verify_digest_cb(params, &digest, cb)
    }

    /// Checks the cryptographic digest against the `BlockNode`'s digest.
    pub fn check_digest<HP: Datable>(&self,
                                     params: &HP,
                                     cb: &Fn(&Self, &HP, &D) -> Result<()>)
        -> Result<()>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut bn = self.clone();
        bn.id = D::default();
        bn.update_size();

        bn.check_digest_cb(params, &digest, cb)
    }

    /// Commits cryptographically the `BlockNode`.
    pub fn commit<CP, C>(&self, params: &CP, cb: &Fn(&Self, &CP) -> Result<C>)
        -> Result<C>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;

        self.commit_cb(params, cb)
    }

    /// Verifies the cryptographic commitment against the `BlockNode`'s commitment.
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

    /// Checks the cryptographic commitment against the `BlockNode`'s commitment.
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

    /// Authenticates cryptographically the `BlockNode`.
    pub fn authenticate<AP, T>(&self, params: &AP, cb: &Fn(&Self, &AP) -> Result<T>)
        -> Result<T>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;

        self.authenticate_cb(params, cb)
    }

    /// Verifies the cryptographic authentication of the `BlockNode` against a tag.
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

    /// Checks the cryptographic authentication of the `BlockNode` against a tag.
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
}

impl<P, D> Hashable<P, D> for BlockNode<D>
    where   P: Datable,
            D: Datable + ConstantSize
{}

impl<CP, C, D> Committable<CP, C> for BlockNode<D>
    where   CP: Datable,
            C: Datable + ConstantSize,
            D: Datable + ConstantSize
{}

impl<AP, T, D> Authenticatable<AP, T> for BlockNode<D>
    where   AP: Datable,
            T: Datable + ConstantSize,
            D: Datable + ConstantSize
{}

impl<D> Sizable for BlockNode<D>
    where   D: Datable + ConstantSize
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.block_id.size() +
            self.block_height.size()
    }
}

impl<D> Checkable for BlockNode<D>
    where   D: Datable + ConstantSize
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
    where   D: Datable + ConstantSize + Serializable
{}

impl<D> Datable for BlockNode<D>
    where   D: Datable + ConstantSize
{}

impl<S, D> Storable<S, D, BlockNode<D>> for BlockNode<D>
    where   S: Datable,
            D: Datable + ConstantSize
{}

impl<S, NA, NP, D> Networkable<S, NA, NP, D, BlockNode<D>> for BlockNode<D>
    where   S: Datable,
            NA: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize
{}