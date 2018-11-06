//! # Output
//!
//! `output` is the module providing the type used to represent the output of a `Transaction`.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize};
use base::Numerical;
use base::{Eval, EvalMut};
use base::Meta;
use crypto::Hash;
use io::{Store, Storable};

/// Type representing the output of a `Transaction`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Output<D, A, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            P: Datable
{
    /// Output id. It is the digest of the same coin, but with a default `D` id.
    pub id: D,
    /// Output metadata.
    pub meta: Meta,
    /// Output amount.
    pub amount: A,
    /// Custom payload.
    pub payload: P,
}

impl<D, A, P> Output<D, A, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Self: Serializable
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
    pub fn finalize<H: Hash<D>>(mut self, hasher: &mut H) -> Result<Self> {
        let msg = self.to_bytes()?;
        self.id = hasher.digest(&msg)?;

        self.update_size();

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Output`.
    pub fn digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<D> {
        let mut output = self.clone();
        output.id = D::default();
        output.update_size();

        let msg = output.to_bytes()?;
        hasher.digest(&msg)
    }

    /// Verifies the cryptographic digest against the `Output`'s digest.
    pub fn verify_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<bool> {
        let digest = self.id.clone();
        digest.check()?;

        let mut output = self.clone();
        output.id = D::default();
        output.update_size();

        let msg = output.to_bytes()?;
        hasher.verify(&msg, &digest)
    }

    /// Checks the cryptographic digest against the `Output`'s digest.
    pub fn check_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<()> {
        let digest = self.id.clone();
        digest.check()?;

        let mut output = self.clone();
        output.id = D::default();
        output.update_size();

        let msg = output.to_bytes()?;
        hasher.check(&msg, &digest)
    }

    /// Evals the `Output`.
    pub fn eval<Ev, EP, ER>(&self, params: &EP, evaluator: &Ev)
        -> Result<ER>
        where   Ev: Eval<Self, EP, ER>,
                EP: Datable,
                ER: Datable
    {
        self.check()?;
        params.check()?;

        evaluator.eval(self, params)
    }

    /// Evals mutably the `Output`.
    pub fn eval_mut<EvM, EP, ER>(&mut self, params: &EP, evaluator: &mut EvM)
        -> Result<ER>
        where   EvM: EvalMut<Self, EP, ER>,
                EP: Datable,
                ER: Datable
    {
        self.check()?;
        params.check()?;

        let result = evaluator.eval_mut(self, params)?;
        self.update_size();

        self.check()?;

        Ok(result)
    }
}

impl<D, A, P> Sizable for Output<D, A, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            P: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.amount.size() +
            self.payload.size()
    }
}

impl<D, A, P> Checkable for Output<D, A, P>
    where   D: Ord + Datable + ConstantSize,
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

        self.amount.check()?;
        self.payload.check()?;

        Ok(())
    }
}

impl<D, A, P> Serializable for Output<D, A, P>
    where   D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable
{}

impl<D, A, P> Datable for Output<D, A, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            P: Datable
{}

pub const OUTPUT_STORE_PREFIX: u64 = 2;

impl<St, S, D, A, P>
    Storable<St, S, D, Output<D, A, P>>
    for Output<D, A, P>
    where   St: Store<S>,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable
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