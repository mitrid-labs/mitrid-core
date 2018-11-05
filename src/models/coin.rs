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
use base::Meta;
use crypto::{Hashable, Committable, Authenticatable};
use io::{Store, Storable};

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
            A: Numerical
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
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>) -> Result<Self>
    {
        params.check()?;

        self.update_size();

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Coin`.
    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>) -> Result<D>
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

    /// Commits cryptographically the `Coin`.
    pub fn commit<CP, C>(&self, params: &CP, cb: &Fn(&Self, &CP) -> Result<C>)
        -> Result<C>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;

        self.commit_cb(params, cb)
    }

    /// Verifies the cryptographic commitment against the `Coin`'s commitment.
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

    /// Checks the cryptographic commitment against the `Coin`'s commitment.
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

    /// Authenticates cryptographically the `Coin`.
    pub fn authenticate<AP, T>(&self, params: &AP, cb: &Fn(&Self, &AP) -> Result<T>)
        -> Result<T>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;

        self.authenticate_cb(params, cb)
    }

    /// Verifies the cryptographic authentication of the `Coin` against a tag.
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

    /// Checks the cryptographic authentication of the `Coin` against a tag.
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
}

impl<P, D, A> Hashable<P, D> for Coin<D, A>
    where   P: Datable,
            D: Ord + Datable + ConstantSize,
            A: Numerical
{}

impl<CP, C, D, A> Committable<CP, C> for Coin<D, A>
    where   CP: Datable,
            C: Datable + ConstantSize,
            D: Ord + Datable + ConstantSize,
            A: Numerical
{}

impl<AP, T, D, A> Authenticatable<AP, T> for Coin<D, A>
    where   AP: Datable,
            T: Datable + ConstantSize,
            D: Ord + Datable + ConstantSize,
            A: Numerical
{}

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

pub const COIN_STORE_PREFIX: u64 = 0;

impl<St, S, D, A, StPC, StRC>
    Storable<St, S, D, Coin<D, A>, StPC, StRC>
    for Coin<D, A>
    where   St: Store<S, StPC, StRC>,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            StPC: Datable + Serializable,
            StRC: Datable + Serializable
{
    fn store_prefix() -> u64 {
        COIN_STORE_PREFIX
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