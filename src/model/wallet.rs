//! # Wallet
//!
//! `wallet` is the module providing the type used for wallets (accounts) in the distributed ledger.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize};
use base::{Eval, EvalMut};
use base::Meta;
use crypto::{Hash, Sign};
use io::{Store, Storable};

/// Type used to represent a wallet (account) in the distributed ledger.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Wallet<D, Sk, Pk, Sig, P>
    where   D: Ord + Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{
    /// Wallet id. It is the digest of the same wallet, but with a default `D` id.
    pub id: D,
    /// Wallet metadata.
    pub meta: Meta,
    /// Wallet secret key, if known.
    pub secret_key: Option<Sk>,
    /// Wallet public key.
    pub public_key: Pk,
    /// Custom payload.
    pub payload: P,
    /// Signature of the wallet with a default id.
    pub signature: Sig,
}

impl<D, Sk, Pk, Sig, P> Wallet<D, Sk, Pk, Sig, P>
    where   D: Ord + Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable,
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

    /// Sets the `Wallet`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Self> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Signs cryptographically the `Wallet`.
    pub fn sign<Sgnr, Seed>(mut self, sk: &Sk, pk: &Pk, signer: &mut Sgnr) -> Result<Self>
        where   Sgnr: Sign<Seed, Pk, Sk, Sig>,
                Seed: Datable + ConstantSize
    {
        sk.check()?;
        pk.check()?;

        self.public_key = pk.clone();
        
        let mut wallet = self.clone();
        wallet.signature = Sig::default();
        wallet.id = D::default();

        let msg = wallet.to_bytes()?;
        self.signature = signer.sign(&msg, sk)?;

        self.update_size();

        Ok(self)
    }

    /// Verifies the cryptographic signature against the `Wallet`.
    pub fn verify_signature<Sgnr, Seed>(&self, signer: &mut Sgnr) -> Result<bool>
        where   Sgnr: Sign<Seed, Pk, Sk, Sig>,
                Seed: Datable + ConstantSize
    {
        let pk = self.public_key.clone();
        pk.check()?;

        let sig = self.signature.clone();
        sig.check()?;

        let mut wallet = self.clone();
        wallet.signature = Sig::default();
        wallet.id = D::default();
        wallet.update_size();

        let msg = wallet.to_bytes()?;

        signer.verify(&msg, &pk, &sig)
    }

    /// Checks the cryptographic signature against the `Wallet`.
    pub fn check_signature<Sgnr, Seed>(&self, signer: &mut Sgnr) -> Result<()>
        where   Sgnr: Sign<Seed, Pk, Sk, Sig>,
                Seed: Datable + ConstantSize
    {
        let pk = self.public_key.clone();
        pk.check()?;

        let sig = self.signature.clone();
        sig.check()?;

        let mut wallet = self.clone();
        wallet.signature = Sig::default();
        wallet.id = D::default();
        wallet.update_size();

        let msg = wallet.to_bytes()?;

        signer.check(&msg, &pk, &sig)
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

impl<D, Sk, Pk, Sig, P> Sizable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Ord + Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.secret_key.size() +
            self.public_key.size() +
            self.payload.size() +
            self.signature.size()
    }
}

impl<D, Sk, Pk, Sig, P> Checkable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Ord + Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.get_size() != self.size() {
            return Err(String::from("invalid meta size"));
        }
        
        if let Some(ref sk) = self.secret_key {
            sk.check()?;
            sk.check_size()?;
        }

        self.public_key.check()?;
        self.public_key.check_size()?;
        self.payload.check()?;
        self.signature.check()?;
        self.signature.check_size()?;

        Ok(())
    }
}

impl<D, Sk, Pk, Sig, P> Serializable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Ord + Datable + ConstantSize + Serializable,
            Sk: Datable + ConstantSize + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable,
            P: Datable + Serializable
{}

impl<D, Sk, Pk, Sig, P> Datable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Ord + Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{}

pub const WALLET_STORE_PREFIX: u64 = 7;

impl<St, S, D, Sk, Pk, Sig, P>
    Storable<St, S, D, Wallet<D, Sk, Pk, Sig, P>>
    for Wallet<D, Sk, Pk, Sig, P>
    where   St: Store<S>,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            Sk: Datable + ConstantSize + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable,
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