use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Runnable;
use crypto::Hashable;
use models::Meta;
use models::BlockNode;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct BlockGraph<D, P>
    where   D: Datable + FixedSize,
            P: Datable
{
    pub id: D,
    pub meta: Meta,
    pub height: u64,
    pub tip: Option<BlockNode<D>>,
    pub frontier_len: u64,
    pub frontier: Vec<BlockNode<D>>,
    pub payload: P,
}

impl<D, P> BlockGraph<D, P>
    where   D: Datable + FixedSize,
            P: Datable
{
    pub fn new() -> BlockGraph<D, P> {
        BlockGraph::default()
    }

    pub fn meta(mut self, meta: &Meta) -> Result<BlockGraph<D, P>> {
        meta.check()?;
        self.meta = meta.clone();

        Ok(self)
    }

    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<BlockGraph<D, P>>
    {
        params.check()?;

        self.id = self.digest_cb(params, cb)?;

        self.check()?;

        Ok(self)
    }
}

impl<RP, D, P> Runnable<RP, D> for BlockGraph<D, P>
    where   RP: Datable,
            D: Datable + FixedSize,
            P: Datable
{}

impl<HP, D, P> Hashable<HP, D> for BlockGraph<D, P>
    where   HP: Datable,
            D: Datable + FixedSize,
            P: Datable
{}

impl<D, P> Sizable for BlockGraph<D, P>
    where   D: Datable + FixedSize,
            P: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.height.size() +
            self.tip.size() +
            self.frontier_len.size() +
            self.frontier.size() +
            self.payload.size()
    }
}

impl<D, P> Checkable for BlockGraph<D, P>
    where   D: Datable + FixedSize,
            P: Datable
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.size != self.size() {
            return Err(String::from("invalid meta size"));
        }
        
        self.height.check()?;
        self.tip.check()?;
        self.frontier_len.check()?;

        if self.frontier.len() != self.frontier_len as usize {
            return Err(String::from("invalid frontier length"));
        }
        
        for node in self.frontier.clone() {
            node.check()?;
        }

        self.payload.check()?;

        Ok(())
    }
}

impl<D, P> Serializable for BlockGraph<D, P>
    where   D: Datable + FixedSize + Serializable,
            P: Datable + Serializable
{}

impl<D, P> Datable for BlockGraph<D, P>
    where   D: Datable + FixedSize,
            P: Datable
{}