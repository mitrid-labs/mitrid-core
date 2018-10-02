use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use crypto::Hashable;
use models::Meta;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct BlockNode<D>
    where   D: Datable + FixedSize
{
    pub id: D,
    pub meta: Meta,
    pub block_id: D,
    pub block_height: u64
}

impl<D> BlockNode<D>
    where   D: Datable + FixedSize
{
    pub fn new() -> BlockNode<D> {
        BlockNode::default()
    }

    pub fn meta(mut self, meta: &Meta) -> Result<BlockNode<D>> {
        meta.check()?;
        self.meta = meta.clone();

        Ok(self)
    }

    pub fn block_data(mut self, block_id: &D, block_height: u64)
        -> Result<BlockNode<D>>
    {
        block_id.check()?;
        block_id.check_size()?;

        self.block_id = block_id.clone();
        self.block_height = block_height;

        Ok(self)
    }

    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<BlockNode<D>>
    {
        params.check()?;

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<D>
    {
        params.check()?;

        self.digest_cb(params, cb)
    }

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
        self.block_id.check()?;
        self.block_id.check_size()?;
        
        if self.meta.size != self.size() {
            return Err(String::from("invalid meta size"));
        }

        Ok(())
    }
}

impl<D> Serializable for BlockNode<D>
    where   D: Datable + FixedSize + Serializable
{}

impl<D> Datable for BlockNode<D>
    where   D: Datable + FixedSize
{}