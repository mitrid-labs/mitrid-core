use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Numerical;
use base::Runnable;
use crypto::Hashable;
use models::Meta;
use models::Coin;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{
    pub id: D,
    pub meta: Meta,
    pub coins_len: u64,
    pub coins: Vec<Coin<D, A>>,
    pub payload: P,
    pub public_key: Pk,
    pub signature: Sig,
}

impl<RP, D, A, P, Pk, Sig> Runnable<RP, D> for Input<D, A, P, Pk, Sig>
    where   RP: Datable,
            D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{}

impl<HP, D, A, P, Pk, Sig> Hashable<HP, D> for Input<D, A, P, Pk, Sig>
    where   HP: Datable,
            D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{}

impl<D, A, P, Pk, Sig> Sizable for Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.coins_len.size() +
            self.coins.size() +
            self.payload.size() +
            self.public_key.size() +
            self.signature.size()
    }
}

impl<D, A, P, Pk, Sig> Checkable for Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.size != self.size() {
            return Err(String::from("invalid meta size"));
        }
        
        self.coins_len.check()?;
        self.coins.check()?;

        if self.coins.len() != self.coins_len as usize {
            return Err(String::from("invalid coins length"));
        }

        self.payload.check()?;
        self.public_key.check()?;
        self.public_key.check_size()?;
        self.signature.check()?;
        self.signature.check_size()?;

        Ok(())
    }
}

impl<D, A, P, Pk, Sig> Serializable for Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable,
            Pk: Datable + FixedSize + Serializable,
            Sig: Datable + FixedSize + Serializable
{}

impl<D, A, P, Pk, Sig> Datable for Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{}