//! # Coin
//!
//! `coin` is the module providing the `Coin` type, an `Output` already registered or sent to the
//! distributed ledger, or just past in time.

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

/// Code of the `Coin` type.
pub const COIN_CODE: u64 = 0;

/// Type used to represent a past `Output`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Coin<D, A>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical
{
    /// Coin id. It is the digest of the same coin, but with a default `D` id.
    pub id: D,
    /// Coin metadata.
    pub meta: Meta,
    /// Coin's `Output` `Transaction`'s id.
    pub tx_id: D,
    /// Coin`'s `Output` index in the `Transaction`'s outputs field.
    pub out_idx: u64,
    /// Coin`'s `Output` amount.
    pub out_amount: A,
}

impl<D, A> Coin<D, A>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            Self: Serializable
{
    /// Creates a new `Coin`.
    pub fn new() -> Self {
        let mut coin = Coin::default();
        coin.update_size();
        coin
    }

    /// Updates the `Coin` size.
    pub fn update_size(&mut self) {
        let size = self.size();

        self.meta.set_size(size);
    }

    /// Sets the `Coin`'s metadata.
    pub fn meta(mut self, meta: &Meta) -> Result<Self> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Coin`'s output data (tx_id, out_idx, out_amount).
    pub fn output_data(mut self, tx_id: &D, out_idx: u64, out_amount: &A) -> Result<Self>
    {
        tx_id.check()?;
        tx_id.check_size()?;
        out_amount.check()?;

        self.tx_id = tx_id.clone();
        self.out_idx = out_idx;
        self.out_amount = out_amount.clone();

        self.update_size();

        Ok(self)
    }

    /// Finalizes the `Coin`, building its id and returning it's complete form.
    pub fn finalize<H: Hash<D>>(mut self, hasher: &mut H) -> Result<Self> {
        let msg = self.to_bytes()?;
        self.id = hasher.digest(&msg)?;

        self.update_size();

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Coin`.
    pub fn digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<D> {
        let mut coin = self.clone();
        coin.id = D::default();
        coin.update_size();

        let msg = coin.to_bytes()?;
        hasher.digest(&msg)
    }

    /// Verifies the cryptographic digest against the `Coin`'s digest.
    pub fn verify_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<bool> {
        let digest = self.id.clone();
        digest.check()?;

        let mut coin = self.clone();
        coin.id = D::default();
        coin.update_size();

        let msg = coin.to_bytes()?;
        hasher.verify(&msg, &digest)
    }

    /// Checks the cryptographic digest against the `Coin`'s digest.
    pub fn check_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<()> {
        let digest = self.id.clone();
        digest.check()?;

        let mut coin = self.clone();
        coin.id = D::default();
        coin.update_size();

        let msg = coin.to_bytes()?;
        hasher.check(&msg, &digest)
    }

    /// Evals the `Coin`.
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

    /// Evals mutably the `Coin`.
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

impl<D, A> Sizable for Coin<D, A>
    where   D: Ord + Datable + ConstantSize,
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
    where   D: Ord + Datable + ConstantSize,
            A: Numerical
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.get_size() != self.size() {
            return Err(String::from("invalid meta size"));
        }
        
        self.tx_id.check()?;
        self.tx_id.check_size()?;
        self.out_amount.check()?;

        Ok(())
    }
}

impl<D, A> Serializable for Coin<D, A>
    where   D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable
{}

impl<D, A> Datable for Coin<D, A>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical
{}

impl<St, S, D, A>
    Storable<St, S, D, Coin<D, A>>
    for Coin<D, A>
    where   St: Store<S>,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable
{
    fn store_prefix() -> Vec<u8> {
        let mut prefix = Vec::new();

        let _prefix: [u8; 8] = unsafe { mem::transmute(COIN_CODE) };
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