//! # Output
//!
//! `output` is the module providing the type used to represent the output of a `Transaction`.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Numerical;
use base::Evaluable;
use crypto::Hashable;
use models::Meta;

/// Type representing the output of a `Transaction`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Output<D, Pk, A, P>
    where   D: Datable + FixedSize,
            Pk: Datable + FixedSize,
            A: Numerical,
            P: Datable
{
    /// Output id. It is the digest of the same coin, but with a default `D` id.
    pub id: D,
    /// Output metadata.
    pub meta: Meta,
    /// Output sender.
    pub sender: Pk,
    /// Output receiver.
    pub receiver: Pk,
    /// Output amount.
    pub amount: A,
    /// Custom payload.
    pub payload: P,
}

impl<D, Pk, A, P> Output<D, Pk, A, P>
    where   D: Datable + FixedSize,
            Pk: Datable + FixedSize,
            A: Numerical,
            P: Datable
{
    /// Creates a new `Output`.
    pub fn new() -> Output<D, Pk, A, P> {
        let mut output = Output::default();
        output.update_size();
        output
    }

    /// Updates the `Output` size.
    pub fn update_size(&mut self) {
        let size = self.size();

        self.meta.set_size(size);
    }

    /// Sets the `Output`'s metadata.
    pub fn meta(mut self, meta: &Meta) -> Result<Output<D, Pk, A, P>> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Output`'s sender.
    pub fn sender(mut self, sender: &Pk) -> Result<Output<D, Pk, A, P>> {
        sender.check()?;
        self.sender = sender.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Output`'s receiver.
    pub fn receiver(mut self, receiver: &Pk) -> Result<Output<D, Pk, A, P>> {
        receiver.check()?;
        self.receiver = receiver.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Output`'s amount.
    pub fn amount(mut self, amount: &A) -> Result<Output<D, Pk, A, P>> {
        amount.check()?;
        self.amount = amount.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Output`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Output<D, Pk, A, P>> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Finalizes the `Output`, building its id and returning it's complete form.
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<Output<D, Pk, A, P>>
    {
        params.check()?;

        self.update_size();

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Output`.
    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<D>
    {
        params.check()?;

        self.digest_cb(params, cb)
    }

    /// Verifies the cryptographic digest against the `Output`'s digest.
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

    /// Checks the cryptographic digest against the `Output`'s digest.
    pub fn check_digest<HP: Datable>(&self,
                                     params: &HP,
                                     digest: &D,
                                     cb: &Fn(&Self, &HP, &D) -> Result<()>)
        -> Result<()>
    {
        params.check()?;
        digest.check()?;

        self.check_digest_cb(params, digest, cb)
    }

    /// Evals the `Output`.
    pub fn eval<EP, R>(&self, params: &EP, cb: &Fn(&Self, &EP) -> Result<R>)
        -> Result<R>
        where   EP: Datable,
                R: Datable
    {
        params.check()?;

        self.eval_cb(params, cb)
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
        
        if self.meta.get_size() != self.size() {
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

impl<D, Pk, A, P> Evaluable for Output<D, Pk, A, P>
    where   D: Datable + FixedSize,
            Pk: Datable + FixedSize,
            A: Numerical,
            P: Datable
{}