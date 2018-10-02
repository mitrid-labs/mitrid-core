use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Numerical;
use crypto::Hashable;
use models::Meta;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Coin<D, A>
    where   D: Datable + FixedSize,
            A: Numerical
{
    pub id: D,
    pub meta: Meta,
    pub tx_id: D,
    pub out_idx: u64,
    pub out_amount: A,
}

impl<D, A> Coin<D, A>
    where   D: Datable + FixedSize,
            A: Numerical
{
    pub fn new() -> Coin<D, A> {
        Coin::default()
    }

    pub fn meta(mut self, meta: &Meta) -> Result<Coin<D, A>> {
        meta.check()?;
        self.meta = meta.clone();

        Ok(self)
    }

    pub fn output_data(mut self, tx_id: &D, out_idx: u64, out_amount: &A)
        -> Result<Coin<D, A>>
    {
        tx_id.check()?;
        tx_id.check_size()?;
        out_amount.check()?;

        self.tx_id = tx_id.clone();
        self.out_idx = out_idx;
        self.out_amount = out_amount.clone();

        Ok(self)
    }

    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<Coin<D, A>>
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

impl<P, D, A> Hashable<P, D> for Coin<D, A>
    where   P: Datable,
            D: Datable + FixedSize,
            A: Numerical
{}

impl<D, A> Sizable for Coin<D, A>
    where   D: Datable + FixedSize,
            A: Numerical
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.tx_id.size() +
            self.out_idx.size() +
            self.out_amount.size()
    }
}

impl<D, A> Checkable for Coin<D, A>
    where   D: Datable + FixedSize,
            A: Numerical
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.size != self.size() {
            return Err(String::from("invalid meta size"));
        }
        
        self.tx_id.check()?;
        self.tx_id.check_size()?;
        self.out_amount.check()?;

        Ok(())
    }
}

impl<D, A> Serializable for Coin<D, A>
    where   D: Datable + FixedSize + Serializable,
            A: Numerical + Serializable
{}

impl<D, A> Datable for Coin<D, A>
    where   D: Datable + FixedSize,
            A: Numerical
{}