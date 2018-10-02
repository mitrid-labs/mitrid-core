use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Numerical;
use base::Evaluable;
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

impl<D, Pk, A, P> Output<D, Pk, A, P>
    where   D: Datable + FixedSize,
            Pk: Datable + FixedSize,
            A: Numerical,
            P: Datable
{
    pub fn new() -> Output<D, Pk, A, P> {
        Output::default()
    }

    pub fn meta(mut self, meta: &Meta) -> Result<Output<D, Pk, A, P>> {
        meta.check()?;
        self.meta = meta.clone();

        Ok(self)
    }

    pub fn sender(mut self, sender: &Pk) -> Result<Output<D, Pk, A, P>> {
        sender.check()?;
        self.sender = sender.clone();

        Ok(self)
    }

    pub fn receiver(mut self, receiver: &Pk) -> Result<Output<D, Pk, A, P>> {
        receiver.check()?;
        self.receiver = receiver.clone();

        Ok(self)
    }

    pub fn amount(mut self, amount: &A) -> Result<Output<D, Pk, A, P>> {
        amount.check()?;
        self.amount = amount.clone();

        Ok(self)
    }

    pub fn payload(mut self, payload: &P) -> Result<Output<D, Pk, A, P>> {
        payload.check()?;

        self.payload = payload.clone();

        Ok(self)
    }

    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<Output<D, Pk, A, P>>
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

impl<RP, D, Pk, A, P> Evaluable<RP, D> for Output<D, Pk, A, P>
    where   RP: Datable,
            D: Datable + FixedSize,
            Pk: Datable + FixedSize,
            A: Numerical,
            P: Datable
{}