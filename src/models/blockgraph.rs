use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Evaluable;
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

    pub fn frontier(mut self, tip_idx: Option<u64>, frontier: &Vec<BlockNode<D>>)
        -> Result<BlockGraph<D, P>>
    {
        frontier.check()?;

        let mut height = 0;

        for node in frontier.clone() {
            if node.block_height > height {
                height = node.block_height;
            }
        }

        let mut tip = None;

        if let Some(idx) = tip_idx {
            if idx > (frontier.len() -1) as u64 {
                return Err(String::from("invalid tip index"));
            }

            tip = Some(frontier[idx as usize].clone());
        }

        self.height = height;
        self.tip = tip;
        self.frontier_len = frontier.len() as u64;
        self.frontier = frontier.clone();

        Ok(self)
    }

    pub fn payload(mut self, payload: &P) -> Result<BlockGraph<D, P>> {
        payload.check()?;

        self.payload = payload.clone();

        Ok(self)
    }

    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<BlockGraph<D, P>>
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

impl<D, P> Evaluable for BlockGraph<D, P>
    where   D: Datable + FixedSize,
            P: Datable
{}