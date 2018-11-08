//! # Input
//!
//! `input` is the module providing the type used to bind as inputs one or more `Input`s
//! in a `Input`.

use std::mem;

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize};
use base::{Eval, EvalMut};
use base::Numerical;
use base::Meta;
use crypto::Hash;
use io::{Store, Storable};
use model::Coin;

/// Code of the `Input` type.
pub const INPUT_CODE: u64 = 1;

/// Type used to bind one or more `Input`s to a `Input` as one of its inputs.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Input<D, A, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            P: Datable
{
    /// Input id. It is the digest of the same input, but with a default `D` id.
    pub id: D,
    /// Input metadata.
    pub meta: Meta,
    /// Bound coin.
    pub coin: Coin<D, A>,
    /// Custom payload.
    pub payload: P,
}

impl<D, A, P> Input<D, A, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Self: Serializable
{
    /// Creates a new `Input`.
    pub fn new() -> Self {
        let mut input = Input::default();
        input.update_size();
        input
    }

    /// Updates the `Input` size.
    pub fn update_size(&mut self) {
        let size = self.size();

        self.meta.set_size(size);
    }

    /// Sets the `Input`'s metadata.
    pub fn meta(mut self, meta: &Meta) -> Result<Self> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Input`s coin.
    pub fn coin(mut self, coin: &Coin<D, A>,) -> Result<Self> {
        coin.check()?;

        self.coin = coin.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Input`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Self> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Finalizes the `Input`, building its id and returning it's complete form.
    pub fn finalize<H: Hash<D>>(mut self, hasher: &mut H) -> Result<Self> {
        let msg = self.to_bytes()?;
        self.id = hasher.digest(&msg)?;

        self.update_size();

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Input`.
    pub fn digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<D> {
        let mut input = self.clone();
        input.id = D::default();
        input.update_size();

        let msg = input.to_bytes()?;
        hasher.digest(&msg)
    }

    /// Verifies the cryptographic digest against the `Input`'s digest.
    pub fn verify_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<bool> {
        let digest = self.id.clone();
        digest.check()?;

        let mut input = self.clone();
        input.id = D::default();
        input.update_size();

        let msg = input.to_bytes()?;
        hasher.verify(&msg, &digest)
    }

    /// Checks the cryptographic digest against the `Input`'s digest.
    pub fn check_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<()> {
        let digest = self.id.clone();
        digest.check()?;

        let mut input = self.clone();
        input.id = D::default();
        input.update_size();

        let msg = input.to_bytes()?;
        hasher.check(&msg, &digest)
    }

    /// Evals the `Input`.
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

    /// Evals mutably the `Input`.
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

impl<D, A, P> Sizable for Input<D, A, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            P: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.coin.size() +
            self.payload.size()
    }
}

impl<D, A, P> Checkable for Input<D, A, P>
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
        
        self.coin.check()?;

        self.payload.check()?;

        Ok(())
    }
}

impl<D, A, P> Serializable for Input<D, A, P>
    where   D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable
{}

impl<D, A, P> Datable for Input<D, A, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            P: Datable
{}

impl<St, S, D, A, P>
    Storable<St, S, D, Input<D, A, P>>
    for Input<D, A, P>
    where   St: Store<S>,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable
{
    fn store_prefix() -> Vec<u8> {
        let mut prefix = Vec::new();

        let _prefix: [u8; 8] = unsafe { mem::transmute(INPUT_CODE) };
        prefix.extend_from_slice(&_prefix[..]);

        prefix
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