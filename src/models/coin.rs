//! # Coin
//!
//! `coin` is the module providing the `Coin` type, an `Output` already registered or sent to the
//! distributed ledger, or just past in time.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize};
use base::Numerical;
use crypto::{Hashable, Committable};
use models::Meta;

/// Type used to represent a past `Output`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Coin<D, A>
    where   D: Datable + ConstantSize,
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
    where   D: Datable + ConstantSize,
            A: Numerical
{
    /// Creates a new `Coin`.
    pub fn new() -> Coin<D, A> {
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
    pub fn meta(mut self, meta: &Meta) -> Result<Coin<D, A>> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Coin`'s output data (tx_id, out_idx, out_amount).
    pub fn output_data(mut self, tx_id: &D, out_idx: u64, out_amount: &A)
        -> Result<Coin<D, A>>
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
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<Coin<D, A>>
    {
        params.check()?;

        self.update_size();

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Coin`.
    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<D>
    {
        params.check()?;

        let mut coin = self.clone();
        coin.id = D::default();

        coin.digest_cb(params, cb)
    }

    /// Verifies the cryptographic digest against the `Coin`'s digest.
    pub fn verify_digest<HP: Datable>(&self,
                                      params: &HP,
                                      cb: &Fn(&Self, &HP, &D) -> Result<bool>)
        -> Result<bool>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut coin = self.clone();
        coin.id = D::default();
        coin.update_size();

        coin.verify_digest_cb(params, &digest, cb)
    }

    /// Checks the cryptographic digest against the `Coin`'s digest.
    pub fn check_digest<HP: Datable>(&self,
                                     params: &HP,
                                     cb: &Fn(&Self, &HP, &D) -> Result<()>)
        -> Result<()>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut coin = self.clone();
        coin.id = D::default();
        coin.update_size();

        coin.check_digest_cb(params, &digest, cb)
    }
}

impl<P, D, A> Hashable<P, D> for Coin<D, A>
    where   P: Datable,
            D: Datable + ConstantSize,
            A: Numerical
{}

impl<CP, C, D, A> Committable<CP, C> for Coin<D, A>
    where   CP: Datable,
            C: Datable + ConstantSize,
            D: Datable + ConstantSize,
            A: Numerical
{}

impl<D, A> Sizable for Coin<D, A>
    where   D: Datable + ConstantSize,
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
    where   D: Datable + ConstantSize,
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
    where   D: Datable + ConstantSize + Serializable,
            A: Numerical + Serializable
{}

impl<D, A> Datable for Coin<D, A>
    where   D: Datable + ConstantSize,
            A: Numerical
{}