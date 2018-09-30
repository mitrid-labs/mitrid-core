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
{}

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