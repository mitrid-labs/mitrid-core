use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Numerical;
use base::Runnable;
use crypto::Hashable;
use models::Meta;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Output<D, Pk, A, P>
    where   D: Datable + FixedSize,
            Pk: Datable + FixedSize,
            A: Numerical,
            P: Datable
{
    pub id: D,
    pub meta: Meta,
    pub sender: Pk,
    pub receiver: Pk,
    pub amount: A,
    pub payload: P,
}

impl<RP, D, Pk, A, P> Runnable<RP, D> for Output<D, Pk, A, P>
    where   RP: Datable,
            D: Datable + FixedSize,
            Pk: Datable + FixedSize,
            A: Numerical,
            P: Datable
{}

impl<HP, D, Pk, A, P> Hashable<HP, D> for Output<D, Pk, A, P>
    where   HP: Datable,
            D: Datable + FixedSize,
            Pk: Datable + FixedSize,
            A: Numerical,
            P: Datable
{}

impl<D, Pk, A, P> Sizable for Output<D, Pk, A, P>
    where   D: Datable + FixedSize,
            Pk: Datable + FixedSize,
            A: Numerical,
            P: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.sender.size() +
            self.receiver.size() +
            self.amount.size() +
            self.payload.size()
    }
}

impl<D, Pk, A, P> Checkable for Output<D, Pk, A, P>
    where   D: Datable + FixedSize,
            Pk: Datable + FixedSize,
            A: Numerical,
            P: Datable
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.size != self.size() {
            return Err(String::from("invalid meta size"));
        }
        
        self.sender.check()?;
        self.sender.check_size()?;
        self.receiver.check()?;
        self.receiver.check_size()?;
        self.amount.check()?;
        self.payload.check()?;

        Ok(())
    }
}

impl<D, Pk, A, P> Serializable for Output<D, Pk, A, P>
    where   D: Datable + FixedSize + Serializable,
            Pk: Datable + FixedSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable
{}

impl<D, Pk, A, P> Datable for Output<D, Pk, A, P>
    where   D: Datable + FixedSize,
            Pk: Datable + FixedSize,
            A: Numerical,
            P: Datable
{}