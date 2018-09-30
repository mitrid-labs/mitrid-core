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