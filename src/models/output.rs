//! # Output
//!
//! `output` is the module providing the type used to represent the output of a `Transaction`.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize};
use base::Numerical;
use base::Evaluable;
use base::Meta;
use crypto::{Hashable, Committable, Authenticatable};
use io::{Store, Storable};

/// Type representing the output of a `Transaction`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Output<D, Pk, A, P>
    where   D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
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
    where   D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            A: Numerical,
            P: Datable
{
    /// Creates a new `Output`.
    pub fn new() -> Self {
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
    pub fn meta(mut self, meta: &Meta) -> Result<Self> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Output`'s sender.
    pub fn sender(mut self, sender: &Pk) -> Result<Self> {
        sender.check()?;
        self.sender = sender.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Output`'s receiver.
    pub fn receiver(mut self, receiver: &Pk) -> Result<Self> {
        receiver.check()?;
        self.receiver = receiver.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Output`'s amount.
    pub fn amount(mut self, amount: &A) -> Result<Self> {
        amount.check()?;
        self.amount = amount.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Output`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Self> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Finalizes the `Output`, building its id and returning it's complete form.
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>) -> Result<Self>
    {
        params.check()?;

        self.update_size();

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Output`.
    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>) -> Result<D>
    {
        params.check()?;

        let mut output = self.clone();
        output.id = D::default();

        output.digest_cb(params, cb)
    }

    /// Verifies the cryptographic digest against the `Output`'s digest.
    pub fn verify_digest<HP: Datable>(&self,
                                      params: &HP,
                                      cb: &Fn(&Self, &HP, &D) -> Result<bool>)
        -> Result<bool>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut output = self.clone();
        output.id = D::default();
        output.update_size();

        output.verify_digest_cb(params, &digest, cb)
    }

    /// Checks the cryptographic digest against the `Output`'s digest.
    pub fn check_digest<HP: Datable>(&self,
                                     params: &HP,
                                     cb: &Fn(&Self, &HP, &D) -> Result<()>)
        -> Result<()>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut output = self.clone();
        output.id = D::default();
        output.update_size();

        output.check_digest_cb(params, &digest, cb)
    }

    /// Commits cryptographically the `Output`.
    pub fn commit<CP, C>(&self, params: &CP, cb: &Fn(&Self, &CP) -> Result<C>)
        -> Result<C>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;

        self.commit_cb(params, cb)
    }

    /// Verifies the cryptographic commitment against the `Output`'s commitment.
    pub fn verify_commitment<CP, C>(&self,
                                    params: &CP,
                                    commitment: &C,
                                    cb: &Fn(&Self, &CP, &C) -> Result<bool>)
        -> Result<bool>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;
        commitment.check()?;

        self.verify_commitment_cb(params, commitment, cb)
    }

    /// Checks the cryptographic commitment against the `Output`'s commitment.
    pub fn check_commitment<CP, C>(&self,
                                   params: &CP,
                                   commitment: &C,
                                   cb: &Fn(&Self, &CP, &C) -> Result<()>)
        -> Result<()>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;
        commitment.check()?;

        self.check_commitment_cb(params, commitment, cb)
    }

    /// Authenticates cryptographically the `Output`.
    pub fn authenticate<AP, T>(&self, params: &AP, cb: &Fn(&Self, &AP) -> Result<T>)
        -> Result<T>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;

        self.authenticate_cb(params, cb)
    }

    /// Verifies the cryptographic authentication of the `Output` against a tag.
    pub fn verify_tag<AP, T>(&self,
                             params: &AP,
                             tag: &T,
                             cb: &Fn(&Self, &AP, &T) -> Result<bool>)
        -> Result<bool>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;
        tag.check()?;

        self.verify_tag_cb(params, tag, cb)
    }

    /// Checks the cryptographic authentication of the `Output` against a tag.
    pub fn check_tag<AP, T>(&self,
                            params: &AP,
                            tag: &T,
                            cb: &Fn(&Self, &AP, &T) -> Result<()>)
        -> Result<()>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;
        tag.check()?;

        self.check_tag_cb(params, tag, cb)
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
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            A: Numerical,
            P: Datable
{}

impl<CP, C, D, Pk, A, P> Committable<CP, C> for Output<D, Pk, A, P>
    where   CP: Datable,
            C: Datable + ConstantSize,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            A: Numerical,
            P: Datable
{}

impl<AP, T, D, Pk, A, P> Authenticatable<AP, T> for Output<D, Pk, A, P>
    where   AP: Datable,
            T: Datable + ConstantSize,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            A: Numerical,
            P: Datable
{}

impl<D, Pk, A, P> Sizable for Output<D, Pk, A, P>
    where   D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
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
    where   D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
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
    where   D: Ord + Datable + ConstantSize + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable
{}

impl<D, Pk, A, P> Datable for Output<D, Pk, A, P>
    where   D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            A: Numerical,
            P: Datable
{}

impl<D, Pk, A, P> Evaluable for Output<D, Pk, A, P>
    where   D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            A: Numerical,
            P: Datable
{}

pub const OUTPUT_STORE_PREFIX: u64 = 2;

impl<St, S, D, Pk, A, P, StPC, StRC>
    Storable<St, S, D, Output<D, Pk, A, P>, StPC, StRC>
    for Output<D, Pk, A, P>
    where   St: Store<S, StPC, StRC>,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable,
            StPC: Datable + Serializable,
            StRC: Datable + Serializable
{
    fn store_prefix() -> u64 {
        OUTPUT_STORE_PREFIX
    }

    fn store_key(&self) -> Result<D> {
        self.id.check()?;

        Ok(self.id.clone())
    }

    fn store_value(&self) -> Result<Self> {
        self.check()?;

        Ok(self.clone())
    }
}