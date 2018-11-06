//! # Wallet
//!
//! `wallet` is the module providing the type used for wallets (accounts) in the distributed ledger.

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

/// Type used to represent a wallet (account) in the distributed ledger.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Wallet<D, A, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            P: Datable
{
    /// Wallet id. It is the digest of the same wallet, but with a default `D` id.
    pub id: D,
    /// Wallet metadata.
    pub meta: Meta,
    /// Wallet spent coins length.
    pub spent_coins_len: u64,
    /// Wallet spent coins.
    pub spent_coins: Vec<Coin<D, A>>,
    /// Wallet unspent coins length.
    pub unspent_coins_len: u64,
    /// Wallet unspent coins.
    pub unspent_coins: Vec<Coin<D, A>>,
    /// Custom payload.
    pub payload: P,
}

impl<D, A, P> Wallet<D, A, P>
    where   D: Ord + Datable + ConstantSize,
            P: Datable,
            A: Numerical,
            Self: Serializable
{
    /// Creates a new `Wallet`.
    pub fn new() -> Self {
        let mut wallet = Wallet::default();
        wallet.update_size();
        wallet
    }

    /// Updates the `Wallet` size.
    pub fn update_size(&mut self) {
        let size = self.size();

        self.meta.set_size(size);
    }

    /// Sets the `Wallet`'s metadata.
    pub fn meta(mut self, meta: &Meta) -> Result<Self> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Wallet`s set of spent `Coin`s and its lenght.
    pub fn spent_coins(mut self, spent_coins: &Vec<Coin<D, A>>) -> Result<Self> {
        spent_coins.check()?;

        for ref coin in spent_coins {
            if self.unspent_coins.contains(&coin) {
                return Err(format!("spent and unpent not disjunct"))
            }
        }

        self.spent_coins_len = spent_coins.len() as u64;
        self.spent_coins = spent_coins.clone();

        self.update_size();

        Ok(self)
    }

    /// Adds a spent `Coin` to the `Wallet` set of spent `Coin`s.
    pub fn add_spent_coin(&mut self, spent_coin: &Coin<D, A>) -> Result<()> {
        spent_coin.check()?;
        
        if self.spent_coins.contains(spent_coin) {
            return Err(format!("already found"));
        }

        if self.unspent_coins.contains(spent_coin) {
            return Err(format!("already found"));
        }

        self.spent_coins.push(spent_coin.to_owned());
        self.spent_coins_len += 1;

        Ok(())
    }

    /// Removes a spent `Coin` from the `Wallet` set of spent `Coin`s.
    pub fn del_spent_coin(&mut self, spent_coin: &Coin<D, A>) -> Result<()> {
        spent_coin.check()?;
        
        if !self.spent_coins.contains(spent_coin) {
            return Err(format!("not found"));
        }

        if self.unspent_coins.contains(spent_coin) {
            return Err(format!("already found"));
        }

        let idx = self.spent_coins.binary_search(spent_coin)
                    .map_err(|e| format!("{:?}", e))?;

        self.spent_coins.remove(idx);
        self.spent_coins_len -= 1;

        Ok(())
    }

    /// Sets the `Wallet`s set of unspent `Coin`s and its lenght.
    pub fn unspent_coins(mut self, unspent_coins: &Vec<Coin<D, A>>) -> Result<Self> {
        unspent_coins.check()?;

        for ref coin in unspent_coins {
            if self.spent_coins.contains(&coin) {
                return Err(format!("spent and unpent not disjunct"))
            }
        }

        self.unspent_coins_len = unspent_coins.len() as u64;
        self.unspent_coins = unspent_coins.clone();

        self.update_size();

        Ok(self)
    }

    /// Adds an unspent `Coin` to the `Wallet` set of unspent `Coin`s.
    pub fn add_unspent_coin(&mut self, unspent_coin: &Coin<D, A>) -> Result<()> {
        unspent_coin.check()?;
        
        if self.unspent_coins.contains(unspent_coin) {
            return Err(format!("already found"));
        }

        if self.spent_coins.contains(unspent_coin) {
            return Err(format!("already found"));
        }

        self.unspent_coins.push(unspent_coin.to_owned());
        self.unspent_coins_len += 1;

        Ok(())
    }

    /// Removes a unspent `Coin` from the `Wallet` set of unspent `Coin`s.
    pub fn del_unspent_coin(&mut self, unspent_coin: &Coin<D, A>) -> Result<()> {
        unspent_coin.check()?;
        
        if !self.unspent_coins.contains(unspent_coin) {
            return Err(format!("not found"));
        }

        if self.spent_coins.contains(unspent_coin) {
            return Err(format!("already found"));
        }

        let idx = self.unspent_coins.binary_search(unspent_coin)
                    .map_err(|e| format!("{:?}", e))?;

        self.unspent_coins.remove(idx);
        self.unspent_coins_len -= 1;

        Ok(())
    }

    /// Sets an unspent `Coin` as spent, removing it from the set of unspent `Coin`s
    /// and adding it to the set of spent `Coin`s.
    pub fn spend_coin(&mut self, unspent_coin: &Coin<D, A>) -> Result<()> {
        self.del_unspent_coin(unspent_coin)?;
        self.add_spent_coin(unspent_coin)?;

        Ok(())
    }

    /// Sets the `Wallet`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Self> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Finalizes the `Wallet`, building its id and returning it's complete form.
    pub fn finalize<H: Hash<D>>(mut self, hasher: &mut H) -> Result<Self> {
        let msg = self.to_bytes()?;
        self.id = hasher.digest(&msg)?;

        self.update_size();

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Wallet`.
    pub fn digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<D> {
        let mut wallet = self.clone();
        wallet.id = D::default();
        wallet.update_size();

        let msg = wallet.to_bytes()?;
        hasher.digest(&msg)
    }

    /// Verifies the cryptographic digest against the `Wallet`'s digest.
    pub fn verify_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<bool> {
        let digest = self.id.clone();
        digest.check()?;

        let mut wallet = self.clone();
        wallet.id = D::default();
        wallet.update_size();

        let msg = wallet.to_bytes()?;
        hasher.verify(&msg, &digest)
    }

    /// Checks the cryptographic digest against the `Wallet`'s digest.
    pub fn check_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<()> {
        let digest = self.id.clone();
        digest.check()?;

        let mut wallet = self.clone();
        wallet.id = D::default();
        wallet.update_size();

        let msg = wallet.to_bytes()?;
        hasher.check(&msg, &digest)
    }

    /// Evals the `Wallet`.
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

    /// Evals mutably the `Wallet`.
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

impl<D, A, P> Sizable for Wallet<D, A, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            P: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.payload.size()
    }
}

impl<D, A, P> Checkable for Wallet<D, A, P>
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

        self.spent_coins_len.check()?;
        self.spent_coins.check()?;

        if self.spent_coins.len() != self.spent_coins_len as usize {
            return Err(String::from("invalid spent coins length"));
        }

        self.unspent_coins_len.check()?;
        self.unspent_coins.check()?;

        if self.unspent_coins.len() != self.unspent_coins_len as usize {
            return Err(String::from("invalid unspent coins length"));
        }

        for ref coin in self.spent_coins.iter() {
            if self.unspent_coins.contains(&coin) {
                return Err(format!("spent and unpent not disjunct"))
            }
        }

        for ref coin in self.unspent_coins.iter() {
            if self.spent_coins.contains(&coin) {
                return Err(format!("spent and unpent not disjunct"))
            }
        }

        self.payload.check()?;

        Ok(())
    }
}

impl<D, A, P> Serializable for Wallet<D, A, P>
    where   D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable
{}

impl<D, A, P> Datable for Wallet<D, A, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            P: Datable
{}

pub const WALLET_STORE_PREFIX: u64 = 7;

impl<St, S, D, A, P>
    Storable<St, S, D, Wallet<D, A, P>>
    for Wallet<D, A, P>
    where   St: Store<S>,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable
{
    fn store_prefix() -> u64 {
        WALLET_STORE_PREFIX
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